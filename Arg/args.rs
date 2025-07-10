use std::clone;
use crate::lib::Arg::error::Error;


//Input type
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputType{
    None,
    Open(String),
    Strict(String, Option<Vec<String>>),
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
    pub input: InputType,
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
