use std::fmt;

#[derive(Debug, Clone)]
pub enum Error{
    Silent,
    NotEnoughArguments,
    ArgumentNotFound(String),
    ArgumentRequirementNotMet(String, Vec<String>),
    WrongInputType(String),
    InputNotGiven(String),
    InputNotFound(String, String),
    InputNotNeeded(String),
}
pub trait Clone{
    fn clone(&self) -> Self;
}


impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Error::Silent => Ok(()),
            Error::ArgumentNotFound(s) => write!(f, "'{}': is not a command!", s),
            Error::ArgumentRequirementNotMet(s, dep) => write!(f, "'{}': Needs requirments -> ({:?})!", s, dep),
            Error::WrongInputType(c) => write!(f, "'{}': Wrong input type!", c),
            Error::NotEnoughArguments => write!(f, "Not enough arguments were given!"),
            Error::InputNotGiven(s) => write!(f, "'{}': Needs an input! No input were given!", s),
            Error::InputNotFound(c, s) => write!(f, "'{}': has no option called '{}'!", c, s),
            Error::InputNotNeeded(s) => write!(f, "'{}': Doesn\'t need any inputs!", s),
        }
    }
}
