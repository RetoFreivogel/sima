extern crate either;
use inkwell;

use std::path::Path;
use source_ast as ast;
use inkwell::types::{StructType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValue, BasicValueEnum};

pub fn print_llvm_ir(ast_module: ast::Module, dest_file: &str) {
    let mut cgu = CodegenUnit::new(&ast_module.id);
    cgu.build_module(ast_module);
    cgu.module.verify(true);
    cgu.module.print_to_file(Path::new(dest_file)).unwrap();
}

#[derive(Debug)]
struct CodegenUnit {
    context: inkwell::context::Context,
    module: inkwell::module::Module,
    builder: inkwell::builder::Builder,
    stack: Vec<BasicValueEnum>,
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
    fn build_module(&mut self, ast_module: ast::Module){
    	ast_module.calc_arieties();
        for (id, fun) in ast_module.functions.iter() {
            let function_type = self.build_function_type(&fun.typ);
            self.module.add_function(&id, &function_type, None);
        }
        for (id, fun) in ast_module.functions.iter(){
            if let Some(ref expr) = fun.expr{
            	let function_value = self.module.get_function(id).unwrap();
		        let basic_block = self.context.append_basic_block(&function_value, "entry");
		        self.builder.position_at_end(&basic_block);
		        self.stack.clear();
		        for p in function_value.params(){
		            self.put(p, 0);
        		}
        		self.build_expression(&expr, 0);
        		if self.stack.is_empty(){
        			self.builder.build_return(None);
        		}else{
        			let ret = &self.stack.pop().unwrap() as &BasicValue;
        			self.builder.build_return(Some(ret));
        		}
            }
        }

    }

    fn put(&mut self, value: BasicValueEnum, depth: usize){
    	let index = self.stack.len() - depth;
    	self.stack.insert(index, value);
    }
    fn take(&mut self, depth: usize) -> BasicValueEnum{
    	let index = self.stack.len() - depth - 1;
    	self.stack.remove(index)
    }

    fn build_expression(&mut self, expr: &ast::Expression, depth: usize) {
        use inkwell::values::BasicValue;
        use source_ast::Expression::*;
        use self::either::Either;

        match *expr{
        	Concat{ref left, ref right} => {
        		self.build_expression(left, depth);
        		self.build_expression(right, depth);
        	}
        	Sidecat{ref left, ref right} => {
        		self.build_expression(left, depth + right.in_ariety());
        		self.build_expression(right, depth);
        	},
            Block { .. } => unimplemented!(),
            StringLiteral(ref lit) => {
                let str_arr = self.builder
                    .build_global_string(&lit, "string")
                    .as_pointer_value();
                let i32_0 = self.context.i32_type().const_int(0, false);
                let str_ptr = self.builder.build_gep(&str_arr, &[&i32_0, &i32_0], "");
                self.put(str_ptr.as_basic_value_enum(), depth);
            }
            Identifier {ref id, ref in_ariety, ..} => {
                let function = self.module.get_function(&id);
                assert!(function.is_some(), "Unknown Identifier '{}'", id);
                let function = function.unwrap();
                let mut args : Vec<Box<BasicValue>> = Vec::new(); 
                for _ in 0..in_ariety.get(){
                	args.push(Box::new(self.take(depth)));
                }
            	let args : Vec<_> = args.iter().map(|t| t.as_ref()).collect();
                let ret = self.builder.build_call(&function, &args, "", false);
                if let Either::Left(val) = ret {
                    self.put(val, depth);
                }
            }
            Number(ref num) => {
            	let i : u64 = num.parse().unwrap();
            	let val = self.context.i32_type().const_int(i, false);
            	self.put(val.as_basic_value_enum(), depth);
            },
    		Duplicate => {
    			let val = self.take(depth).clone();
    			self.put(val, depth);
       			self.put(val, depth);
    		},
    		Discard => {
    			self.take(depth);
    		},
    		Keep => {},
    		Exchange => {
    			let first = self.take(depth);
    			let second = self.take(depth);
    			self.put(first, depth);
    			self.put(second, depth);
    		},
        }
    }

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
