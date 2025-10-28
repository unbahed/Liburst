use std::env::args;
use std::collections::HashMap;
use crate::lib::arg::error::Error;
use crate::lib::arg::type_handler;
use crate::lib::arg::args::{Argm, Input, InputType, InputFormat};


fn get_args() -> Vec<String>{
    let mut argv: Vec<String> = Vec::new();
    for i in args(){
        argv.append(&mut vec![i]);
    }
    return argv;
}

fn is_command(argm: &Vec<Argm>, s: &String) -> bool{
    let mut counter = 0;
    for i in argm {
        if i.name.contains(s){
            counter += 1;
        }
    }

    if counter > 0 {
        return true;
    }
    else{
        return false
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

fn check_type(wanted_type: &InputType, given_input: &String) -> Result<Input, Error>{

    match wanted_type {
        InputType::Bool => {
            match type_handler::to_bool(given_input){
                Ok(inp) => {
                    Ok(Input::Bool(inp))
                },
                Err(_) => {
                    Err(Error::WrongInputType(given_input.clone()))
                }
            }
        },
        InputType::Int => {
            match type_handler::to_int(given_input){
                Ok(inp) => {
                    Ok(Input::Int(inp))
                },
                Err(_) => {
                    Err(Error::WrongInputType(given_input.clone()))
                }
            }
        },
        InputType::Path => {
            match type_handler::to_path(given_input){
                Ok(inp) => {
                    Ok(Input::Path(inp))
                },
                Err(_) => {
                    Err(Error::WrongInputType(given_input.clone()))
                }
            }
        },
        InputType::String => {
            Ok(Input::String(given_input.clone()))
        },
    }
}


fn check_input(arg_list: &Vec<Argm>, expected_arg: &Argm, given_arg: &String, given_input: Option<&String>) -> Result<(Argm, Option<Input>), Error>{
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
                    if is_command(arg_list, given_ok_input){
                        return Err(Error::InputNotFound(expected_arg.name[1].clone(), given_ok_input.clone()));
                    }
                    else{
                        match check_type(&input_type, given_ok_input){
                            Ok(inp) => {
                                return Ok((expected_arg.clone(), Some(inp.clone())));
                            },
                            Err(e) =>{
                                return Err(e);
                            },
                        }
                    }
                },
                Err(_) => {
                    return Err(Error::InputNotGiven(given_arg.clone()));
                },
            }
        },
        InputFormat::Strict(input_type, options_list) => {
            match given_input.ok_or(Error::InputNotGiven(given_arg.clone())){
                Ok(given_ok_input)=>{
                    if options_list.contains(given_ok_input){
                        match check_type(input_type, given_ok_input){
                            Ok(inp) => {
                                return Ok((expected_arg.clone(), Some(inp.clone())));
                            },
                            Err(e) =>{
                                return Err(e);
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


pub fn arg_hlr(arg_list: Vec<Argm>) -> Result<HashMap<Argm, Option<Input>>, Error>{
    let given_args = get_args();
    let mut index = 1;
    let mut instructions: HashMap<Argm, Option<Input>> = HashMap::new();

    while index < given_args.len(){
        if is_command(&arg_list, &given_args[index]){
            match extract_arg_from_list(&arg_list, &given_args[index]){
                Ok(called_arg) => {
                    let mut given_input: Option<&String> = None;

                    if index <= given_args.len()-2{
                        if is_command(&arg_list, &given_args[index+1]){
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
                    match check_input(&arg_list, &called_arg, &given_args[index], given_input){
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
