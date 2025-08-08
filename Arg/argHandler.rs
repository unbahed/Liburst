use std::env::{Args, args};
use std::any::{TypeId, type_name, type_name_of_val};
use std::collections::HashMap;
use std::str::FromStr;
use crate::lib::Arg::error::Error;
use crate::lib::Arg::args::{Argm, Argv, InputType, InputFormat};


fn get_args() -> Vec<String>{
    let mut argv: Vec<String> = Vec::new();
    for i in args(){
        argv.append(&mut vec![i]);
    }
    return argv;
}

fn is_command(s: &String) -> bool{
    let tmp = s.chars().nth(0).unwrap();
    if tmp == '-'{
        return true;
    }
    else{
        return false;
    }
}

fn extract_arg_from_list(arg_list: &Vec<Argm>, given_arg: &String) -> Result<Argm, Error>{
    for i in arg_list{
        if i.name.contains(given_arg){
            return Ok(i.clone());
        }
    }
    return Err(Error::ArgumentNotFound(given_arg.clone()));
}


fn convert_type<T: 'static>(var: T) -> InputType{
    let mut converted_type: InputType = InputType::Char;
    match TypeId::of::<T>(){
        val if val == TypeId::of::<i32>() => {
            converted_type = InputType::Int;
        },
        val if val == TypeId::of::<String>() => {
            converted_type = InputType::String;
        },
        val if val == TypeId::of::<f32>() => {
            converted_type = InputType::Float;
        },
        val if val == TypeId::of::<bool>() => {
            converted_type = InputType::Bool;
        },
        val if val == TypeId::of::<u8>() => {
            converted_type = InputType::Char;
        },
        _ => (),
    }
    return converted_type;
}

//Later implementation
fn check_type(reqInputType: &InputType, given_input: &String) -> bool{
    return true;
}


fn check_input(expected_arg: &Argm, given_arg: &String, given_input: Option<&String>) -> Result<(Argm, Option<String>), Error>{
    match &expected_arg.input{
        InputFormat::None =>{
            if given_input == None{
                return Ok((expected_arg.clone(), None));
            }
            else{
                return Err(Error::InputNotNeeded(given_arg.clone()));
            }
        },
        InputFormat::Open(input_type) => {
            match given_input.ok_or(Error::InputNotGiven(given_arg.clone())){
                Ok(given_ok_input) => {
                    if is_command(given_ok_input){
                        return Err(Error::InputNotFound(expected_arg.name[1].clone(), given_ok_input.clone()));
                    }
                    else{
                        match check_type(&input_type, given_ok_input){
                            true => {
                                return Ok((expected_arg.clone(), Some(given_ok_input.clone())));
                            },
                            false =>{
                                return Err(Error::WrongInputType(expected_arg.name[1].clone()));
                            },
                        }
                    }
                },
                Err(e) => {
                    return Err(Error::InputNotGiven(given_arg.clone()));
                },
            }
        },
        InputFormat::Strict(input_type, options_list) => {
            match given_input.ok_or(Error::InputNotGiven(given_arg.clone())){
                Ok(given_ok_input)=>{
                    if options_list.contains(given_ok_input){
                        match check_type(&input_type, given_ok_input){
                            true => {
                                return Ok((expected_arg.clone(), Some(given_ok_input.clone())));
                            },
                            false =>{
                                return Err(Error::WrongInputType(expected_arg.name[1].clone()));
                            },
                        }
                    }
                    else{
                        return Err(Error::InputNotFound(expected_arg.name[1].clone(), given_ok_input.clone()));
                    }
                },
                Err(e)=>{
                    return Err(e);
                },
            }
        },
    }
}


pub fn arg_hlr(arg_list: Vec<Argm>) -> Result<HashMap<Argm, Option<String>>, Error>{
    let given_args = get_args();
    let mut index = 1;
    let mut instructions: HashMap<Argm, Option<String>> = HashMap::new();

    while index < given_args.len(){
        if is_command(&given_args[index]){
            match extract_arg_from_list(&arg_list, &given_args[index]){
                Ok(called_arg) => {
                    let mut given_input: Option<&String> = None;

                    if index <= given_args.len()-2{
                        if is_command(&given_args[index+1]){
                            given_input = None;
                        }
                        else{
                            given_input = Some(&given_args[index + 1]);
                            index += 1;
                        }
                    }
                    else{
                        given_input = None;
                    }
                    match check_input(&called_arg, &given_args[index], given_input){
                        Ok((arg, input)) => {
                            instructions.insert(arg, input);
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                },
                Err(error) => {
                    return Err(error);
                },
            }
        }
        else{
            return Err(Error::ArgumentNotFound(given_args[index].clone()));
        }
        index += 1;
    }
    return Ok(instructions);
}
