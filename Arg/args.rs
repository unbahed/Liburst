use std::clone;
use crate::lib::Arg::error::Error;


//Input type
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputFormat{
    None,
    Open(InputType),
    Strict(InputType, Vec<String>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputType{
    Char,
    String,
    Int, 
    Float,
    Bool,
}

//Dependency type
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dep{
    None,
    List(Vec<String>),
}

//Arguement structures
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argm{
    pub name: Vec<String>,
    pub input: InputFormat,
    pub dependencies: Dep,
    pub desc: String,
}

pub trait Clone{
    fn clone(&self) -> Self;
}

pub fn new() -> Vec<Argm>{
    return Vec::new();
}

impl Clone for Argm{
    fn clone(&self) -> Self{
        return Clone::clone(self);
    }
}
