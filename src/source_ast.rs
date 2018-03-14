use std::error::Error;
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
        assert!(f.expr.is_some(), "Function '{}' has multiple definitions", id);
        f.expr = Some(expr);
    }
    pub fn add_declaration(&mut self, id: &str, typ: SimaType){
        assert!(self.functions.get(id).is_none(), "Function '{}' has multiple declarations", id);
        self.functions.insert(id.to_string(), Function{
            id: id.to_string(),
            typ,
            expr: None,
        });
    }
}

pub struct Function{
    pub id: String,
    pub typ: SimaType,
    pub expr: Option<Expression>,
}

pub enum SimaType{
    Function{args: Vec<SimaType>, ret: Vec<SimaType>},
    Pointer{target: Box<SimaType>},
    Basic{id: String},
    Opaque,
}
impl SimaType{
    pub fn in_ariety(&self) -> usize{
        match self{
            &SimaType::Function{ref args, ret: _} => args.len(),
            _ => 1,
        }
    }
    pub fn out_ariety(&self) -> usize{
        match self{
            &SimaType::Function{args: _, ref ret} => ret.len(),
            _ => 1,
        }
    }
}

pub enum Expression{
    Concat{left: Box<Expression>, right: Box<Expression>},
    Sidecat{left: Box<Expression>, right: Box<Expression>},
    Block{inner: Box<Expression>},
    StringLiteral(String),
    Number(String),
    Identifier{id: String},
    Duplicate,
    Discard,
    Exchange,
    Keep,
}

impl Expression{
    pub fn in_ariety(&self, module: &Module) -> usize{
        use self::Expression::*;
        use std::cmp::max;
        match *self{
            Concat{ref left, ref right} => {
                let l = left.in_ariety(module);
                let r = right.in_ariety(module);
                l+r-max(r, left.out_ariety(module))
            },
            Sidecat{ref left, ref right} => {
                left.in_ariety(module) + right.in_ariety(module)
            },
            Identifier{ref id} => {
                let f = module.functions.get(id);
                assert!(f.is_some(), "Identifier '{}' is not in Scope of Module '{}'", id, module.id);
                let f = f.unwrap();
                f.typ.in_ariety()
            },
            Block{inner: _} | StringLiteral(_) | Number(_) => 0,
            Duplicate | Discard | Keep => 1,
            Exchange => 2,
        }
    }
    pub fn out_ariety(&self, module: &Module) -> usize{
        use self::Expression::*;
        use std::cmp::max;
        match *self{
            Concat{ref left, ref right} => {
                let l = left.out_ariety(module);
                let r = right.out_ariety(module);
                l+r-max(l, right.in_ariety(module))
            },
            Sidecat{ref left, ref right} => {
                left.out_ariety(module) + right.out_ariety(module)
            },
            Identifier{ref id} => {
                let f = module.functions.get(id);
                assert!(f.is_some(), "Identifier '{}' is not in Scope of Module '{}'", id, module.id);
                let f = f.unwrap();
                f.typ.out_ariety()
            },
            Discard => 0,
            Block{inner: _} | StringLiteral(_) | Number(_) | Keep => 1,
            Duplicate | Exchange => 2,
        }
    }
}

