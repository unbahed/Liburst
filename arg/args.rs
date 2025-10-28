use std::path::PathBuf;

//Input type
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputFormat{
    None,
    Open(InputType),
    Strict(InputType, Vec<String>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputType{
    String,
    Int, 
    Bool,
    Path,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Input{
    String(String),
    Int(i32), 
    Bool(bool),
    Path(PathBuf),
}

//Arguement structures
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argm{
    pub name: Vec<String>,
    pub input: InputFormat,
    pub desc: String,
}
