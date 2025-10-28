use std::path::PathBuf;

use crate::lib::arg::error::Error;

pub fn to_bool(input: &String) -> Result<bool, Error>{

    match input {
        v if v == &"true".to_string() => return Ok(true),
        v if v == &"t".to_string() => return Ok(true),
        v if v == &"T".to_string() => return Ok(true),
        v if v == &"True".to_string() => return Ok(true),
        v if v == &"TRUE".to_string() => return Ok(true),
        v if v == &"false".to_string() => return Ok(false),
        v if v == &"f".to_string() => return Ok(false),
        v if v == &"F".to_string() => return Ok(false),
        v if v == &"False".to_string() => return Ok(false),
        v if v == &"FALSE".to_string() => return Ok(false),
        _ => {
            return Err(Error::WrongInputType(input.clone()));
        }
    };
}

pub fn to_int(input: &String) -> Result<i32, Error>{
    match input.parse::<i32>(){
        Ok(num) => {
            return Ok(num);
        },
        Err(_) => {
            return Err(Error::WrongInputType(input.clone()));
        }
    }

}

pub fn to_path(input: &String) -> Result<PathBuf, Error>{

    let converted = PathBuf::from(input);

    match &converted.exists(){
        true => {
            return Ok(converted);
        },
        false => {
            return Err(Error::WrongInputType(input.clone()));
        },
    }
}
