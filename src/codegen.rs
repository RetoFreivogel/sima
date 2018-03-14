extern crate either;
use inkwell;

use std::path::Path;
use source_ast as ast;
use inkwell::types::{StructType, BasicTypeEnum, FunctionType};

pub fn print_llvm_ir(ast_module: ast::Module, dest_file: &str) {
    let mut cgu = CodegenUnit::new(&ast_module.id);
    cgu.build_module(ast_module);
    cgu.module.verify(true);
    cgu.module.print_to_file(Path::new(dest_file)).unwrap();
}

struct CodegenUnit {
    context: inkwell::context::Context,
    module: inkwell::module::Module,
    builder: inkwell::builder::Builder,
    stack: Vec<inkwell::values::BasicValueEnum>,
    opaque_type: StructType,
}

impl CodegenUnit {
    fn new(name: &str) -> CodegenUnit {
        let context = inkwell::context::Context::create();
        let module = context.create_module(name);
        let builder = context.create_builder();
        let stack = Vec::new();
        let opaque_type = context.opaque_struct_type("opaque");
        CodegenUnit {
            context,
            module,
            builder,
            stack,
            opaque_type,
        }
    }
}

impl CodegenUnit {
    fn build_module(&mut self, ast: ast::Module) -> () {
        for dec in ast.declarations {
            let function_type = self.build_function_type(&dec.sima_type);
            self.module.add_function(&dec.id, &function_type, None);
        }
        for def in ast.definitions {
            //TODO infer function definition type
            self.build_function(def);
        }
    }

    fn build_function(&mut self, def: ast::Definition) {
        use inkwell::values::BasicValue;
        let function = self.module
            .get_function(&def.id)
            .expect("Missing Definition");
        let basic_block = self.context.append_basic_block(&function, "entry");
        self.builder.position_at_end(&basic_block);
        self.stack.clear();
        for i in 0..function.count_params() {
            self.stack.push(function.get_nth_param(i).unwrap());
        }
        /*
        for sym in def.block.symbols {
            self.build_symbol(sym);
        }
        */
        if let Some(ret) = self.stack.pop() {
            self.builder.build_return(Some(&ret as &BasicValue));
        } else {
            self.builder.build_return(None);
        }
    }

/*
    fn build_symbol(&mut self, symbol: ast::Symbol) {
        use inkwell::values::BasicValue;
        use source_ast::Symbol::*;
        use self::either::Either;

        match symbol {
            Block { .. } => unimplemented!(),
            StringLiteral { lit } => {
                let str_arr = self.builder
                    .build_global_string(&lit.value, "")
                    .as_pointer_value();
                let i32_0 = self.context.i32_type().const_int(0, false);
                let str_ptr = self.builder.build_gep(&str_arr, &[&i32_0, &i32_0], "");
                self.stack.push(str_ptr.as_basic_value_enum());
            }
            Identifier { id } => {
                let function = self.module.get_function(&id.name).unwrap();
                let num_args = function.count_params() as usize;
                let stack_len = self.stack.len();
                let args_vec = self.stack.split_off(stack_len - num_args);
                let args = args_vec
                    .iter()
                    .map(|v| v as &BasicValue)
                    .collect::<Vec<&BasicValue>>();

                let ret = self.builder.build_call(&function, &args, "", false);
                if let Either::Left(val) = ret {
                    self.stack.push(val);
                }
            }
            Number { num } => {
            	let val = self.context.i32_type().const_int(num.value, false);
            	self.stack.push(val.as_basic_value_enum());
            },
    		Duplicate => {
    			let last = self.stack.last().expect("dup was used on an empty stack").clone();
    			self.stack.push(last);
    		},
    		Discard => {
    			self.stack.pop().expect("drop was used on an emtpy stack");
    		},
    		Exchange => {
    			let last = self.stack.pop().expect("exchange was used on an empty stack");
    			let second = self.stack.pop().expect("exchange was used on a stack with only one element");
    			self.stack.push(last);
    			self.stack.push(second);
    		},
        }
    }
*/

    fn build_function_type(&mut self, typ: &ast::SimaType) -> FunctionType {
        use source_ast::SimaType::*;
        use inkwell::types::BasicTypeEnum::*;
        use inkwell::types::BasicType;

        if let Function { ref args, ref ret } = *typ {
            //A Day was lost to this Section
            let mut args_box: Vec<Box<BasicType>> = Vec::new();
            for t in args {
                args_box.push(Box::new(self.build_basic_type(t)));
            }
            let args_vec: Vec<&BasicType> = args_box.iter().map(|t| t.as_ref()).collect();
            let args: &[&BasicType] = args_vec.as_slice();

            if ret.is_empty() {
                return self.context.void_type().fn_type(args, false);
            }
            match self.build_basic_type(&ret[0]) {
                IntType(t) => t.fn_type(args, false),
                FloatType(t) => t.fn_type(args, false),
                PointerType(t) => t.fn_type(args, false),
                StructType(t) => t.fn_type(args, false),
                ArrayType(t) => t.fn_type(args, false),
                VectorType(_) => panic!(),
            }
        } else {
            panic!()
        }
    }

    fn build_basic_type(&mut self, typ: &ast::SimaType) -> BasicTypeEnum {
        use source_ast::SimaType::*;
        use inkwell::types::BasicTypeEnum::*;
        match *typ {
            Function { .. } => panic!(),
            Pointer { ref target } => {
                let target = self.build_basic_type(target);
                let address_space = inkwell::AddressSpace::Generic;
                let t = match target {
                    PointerType(t) => t.ptr_type(address_space),
                    IntType(t) => t.ptr_type(address_space),
                    StructType(t) => t.ptr_type(address_space),
                    _ => panic!(),
                };
                BasicTypeEnum::PointerType(t)
            }
            Basic { ref id } => {
                let size: u32 = id[1..].parse().unwrap();
                let t = self.context.custom_width_int_type(size);
                BasicTypeEnum::IntType(t)
            }
            Opaque => {
            	let o = self.opaque_type;
            	BasicTypeEnum::StructType(o)
            }
        }
    }
}
