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
    Identifier(String),
    Duplicate,
    Discard,
    Exchange,
    Keep,
}
