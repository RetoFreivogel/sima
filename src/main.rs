extern crate lalrpop_util;
extern crate inkwell;
mod source_ast;
mod source_grammar;
mod codegen;

use std::env;
use std::path::{Path};
use source_ast::parse_file;
use codegen::print_llvm_ir;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("no input file specified");
        return;
    }
    let filename =  Path::new(&args[1]);
    let mut outname = filename.to_path_buf();
    outname.set_extension("ll");
    let mut outname = outname.to_str().unwrap().to_string();
    outname.shrink_to_fit();
    let module = parse_file(filename).unwrap();
    print_llvm_ir(module, &outname);
}
