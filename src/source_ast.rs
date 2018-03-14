use std::error::Error;
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
    pub declarations: Vec<Declaration>,
    pub definitions: Vec<Definition>,
}
impl Module{
    pub fn new(id: &str) -> Module{
        Module{
            id: id.to_string(),
            declarations: Vec::new(),
            definitions: Vec::new(),
        }
    }
}

pub struct Definition{
    pub id: String,
    pub expr: Expression,
}

pub struct Declaration{
    pub id: String,
    pub sima_type: SimaType,
}

pub enum SimaType{
    Function{args: Vec<SimaType>, ret: Vec<SimaType>},
    Pointer{target: Box<SimaType>},
    Basic{id: String},
    Opaque,
}

pub enum Expression{
    Concat{left: Box<Expression>, right: Box<Expression>},
    Sidecat{left: Box<Expression>, right: Box<Expression>},
    Block{inner: Box<Expression>},
    StringLiteral(String),
    Number(String),
    Identifier{id: String, in_ariety: usize, out_ariety: usize},
    Duplicate,
    Discard,
    Exchange,
    Keep,
}

//TODO infer Identifer ariety

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
            Identifier{id: _, in_ariety, out_ariety: _} => in_ariety,
            Block{inner: _} | StringLiteral(_) | Number(_) => 0,
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
            Identifier{id:_, in_ariety:_, out_ariety} => out_ariety,
            Discard => 0,
            Block{inner: _} | StringLiteral(_) | Number(_) | Keep => 1,
            Duplicate | Exchange => 2,
        }
    }
    pub fn iter_identifers<'a>(&'a mut self) -> Box<'a + Iterator<Item= &'a mut Expression>>{
        use self::Expression::*;
        use std::iter;
        match *self{
            Concat{ref mut left, ref mut right} | Sidecat{ref mut left, ref mut right} => {
                Box::new(left.iter_identifers().chain(right.iter_identifers()))
            },
            Block{ref mut inner} => {
                Box::new(inner.iter_identifers())
            },
            Identifier{id:_, in_ariety:_, out_ariety:_} => {
                Box::new(iter::once(self))
            },
            _ => Box::new(iter::empty())
        }

    }
}
