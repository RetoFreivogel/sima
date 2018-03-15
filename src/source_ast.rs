use std::error::Error;
use std::cell::Cell;
use std::collections::HashMap;
use std::path::Path;

pub fn parse_file(filename: &Path) -> Result<Module, Box<Error>> {
    use std::fs::File;
    use std::io::Read;
    use source_grammar::parse_Module;

    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_Module(&contents).unwrap())
}

#[derive(Debug, Clone)]
pub struct Module{
    pub id: String,
    pub functions: HashMap<String, Function>,
}
impl Module{
    pub fn new(id: &str) -> Module{
        Module{
            id: id.to_string(),
            functions: HashMap::new(),
        }
    }
    pub fn add_definition(&mut self, id: &str, expr: Expression){
        let f = self.functions.get_mut(id);
        assert!(f.is_some(), "Function '{}' must be declared before it gets defined", id);
        let f = f.unwrap();
        assert!(f.expr.is_none(), "Function '{}' has multiple definitions", id);
        f.expr = Some(expr);
    }
    pub fn add_declaration(&mut self, id: &str, typ: SimaType){
        assert!(self.functions.get(id).is_none(), "Function '{}' has multiple declarations", id);
        self.functions.insert(id.to_string(), Function{
            typ,
            expr: None,
        });
    }
    pub fn calc_arieties(&self){
        for (_, fun) in self.functions.iter(){
            if let Some(ref expr) = fun.expr{
                expr.calc_ariety(&self);
            }
        }        
    }
}

#[derive(Debug, Clone)]
pub struct Function{
    pub typ: SimaType,
    pub expr: Option<Expression>,
}
impl Function{
    pub fn in_ariety(&self) -> usize{
        match self.typ{
            SimaType::Function{ref args, ..} => args.len(),
            _ => 1,
        }
    }
    pub fn out_ariety(&self) -> usize{
        match self.typ{
            SimaType::Function{ref ret, ..} => ret.len(),
            _ => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SimaType{
    Function{args: Vec<SimaType>, ret: Vec<SimaType>},
    Pointer{target: Box<SimaType>},
    Basic{id: String},
    Opaque,
}

#[derive(Debug, Clone)]
pub enum Expression{
    Concat{left: Box<Expression>, right: Box<Expression>},
    Sidecat{left: Box<Expression>, right: Box<Expression>},
    Block{inner: Box<Expression>},
    StringLiteral(String),
    Number(String),
    Identifier{id: String, in_ariety: Cell<usize>, out_ariety: Cell<usize>},
    Duplicate,
    Discard,
    Exchange,
    Keep,
}

impl Expression{
    pub fn in_ariety(&self) -> usize{
        use self::Expression::*;
        use std::cmp::max;
        match *self{
            Concat{ref left, ref right} => {
                let l = left.in_ariety();
                let r = right.in_ariety();
                l+r-max(r, left.out_ariety())
            },
            Sidecat{ref left, ref right} => {
                left.in_ariety() + right.in_ariety()
            },
            Identifier{ref in_ariety, ..} => in_ariety.get(),
            Block{..} | StringLiteral(_) | Number(_) => 0,
            Duplicate | Discard | Keep => 1,
            Exchange => 2,
        }
    }
    pub fn out_ariety(&self) -> usize{
        use self::Expression::*;
        use std::cmp::max;
        match *self{
            Concat{ref left, ref right} => {
                let l = left.out_ariety();
                let r = right.out_ariety();
                l+r-max(l, right.in_ariety())
            },
            Sidecat{ref left, ref right} => {
                left.out_ariety() + right.out_ariety()
            },
            Identifier{ref out_ariety, ..} => out_ariety.get(),
            Discard => 0,
            Block{..} | StringLiteral(_) | Number(_) | Keep => 1,
            Duplicate | Exchange => 2,
        }
    }
    pub fn calc_ariety(&self, module: &Module){
        use self::Expression::*;
        match *self{
            Identifier{ref id, ref in_ariety, ref out_ariety} => { 
                let f = module.functions.get(id);
                assert!(f.is_some(), "Identifier '{}' is not in Scope of Module '{}'", id, module.id);
                let f = f.unwrap();
                in_ariety.set(f.in_ariety());
                out_ariety.set(f.out_ariety());
            },
            Concat{ref left, ref right} | Sidecat{ref left, ref right} => {
                left.calc_ariety(module);
                right.calc_ariety(module);
            },
            _ => {},           
        }
    }
}

