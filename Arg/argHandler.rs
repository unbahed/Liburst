use std::env::{Args, args};
use std::any::{TypeId, type_name, type_name_of_val};
use std::collections::HashMap;
use crate::lib::Arg::error::Error;
use crate::lib::Arg::args::{Argm, InputType, InputStriction, Dep};


pub fn get_argv() -> Vec<String>{
    let mut argv: Vec<String> = Vec::new();
    for i in args(){
        argv.append(&mut vec![i]);
    }
    return argv;
}

pub fn is_command(s: &String) -> bool{
    let tmp = s.chars().nth(0).unwrap();
    if tmp == '-'{
        return true;
    }
    else{
        return false;
    }
}

fn check_dep(){
}

fn convert_type<T: 'static>(var: T) -> InputType{
    let mut convertedType: InputType = InputType::Char;
    match TypeId::of::<T>(){
        val if val == TypeId::of::<i32>() => {
            convertedType = InputType::Int;
        },
        val if val == TypeId::of::<String>() => {
            convertedType = InputType::String;
        },
        val if val == TypeId::of::<f32>() => {
            convertedType = InputType::Float;
        },
        val if val == TypeId::of::<bool>() => {
            convertedType = InputType::Bool;
        },
        val if val == TypeId::of::<u8>() => {
            convertedType = InputType::Char;
        },
        _ => (),
    }
    return convertedType;
}

fn check_type(reqInputType: &InputType, givenInput: &String) -> bool{



    return true;
}


fn check_input(arg: &Argm, givenArg: &String, givenInput: Option<&String>) -> Result<(Argm, Option<String>), Error>{
    match &arg.input{
        InputStriction::None =>{
            if givenInput == None{
                return Ok((arg.clone(), None));
            }
            else{
                return Err(Error::InputNotNeeded(givenArg.clone()));
            }
        },
        InputStriction::Open(inputType) => {
            match givenInput.ok_or(Error::InputNotGiven(givenArg.clone())){
                Ok(okGivenInput) => {
                    if is_command(okGivenInput){
                        return Err(Error::InputNotFound(arg.name[1].clone(), okGivenInput.clone()));
                    }
                    else{
                        match check_type(inputType, okGivenInput){
                            true => {
                                return Ok((arg.clone(), Some(okGivenInput.clone())));
                            },
                            false =>{
                                return Err(Error::WrongInputType(arg.name[1].clone()));
                            },
                        }
                    }
                },
                Err(e) => {
                    return Err(Error::InputNotGiven(givenArg.clone()));
                },
            }
        },
        InputStriction::Strict(inputType, optionsList) => {
            match givenInput.ok_or(Error::InputNotGiven(givenArg.clone())){
                Ok(okGivenInput)=>{
                    if optionsList.contains(okGivenInput){
                        match check_type(inputType, okGivenInput){
                            true => {
                                return Ok((arg.clone(), Some(okGivenInput.clone())));
                            },
                            false =>{
                                return Err(Error::WrongInputType(arg.name[1].clone()));
                            },
                        }
                    }
                    else{
                        return Err(Error::InputNotFound(arg.name[1].clone(), okGivenInput.clone()));
                    }
                },
                Err(e)=>{
                    return Err(e);
                },
            }
        },
    }
}


pub fn arg_hlr(argv: &Vec<String>, cmds: &Vec<Argm>) -> Result<HashMap<Argm, Option<String>>, Error>{
    let mut instruction: HashMap<Argm, Option<String>> = HashMap::new();
    match argv.len(){
        0 => Err(Error::NotEnoughArguments),
        1 => Err(Error::NotEnoughArguments),
        _ => {

            let mut i = 1;
            let mut coms = 0;

            while i < argv.len(){
                if is_command(&argv[i]){
                    for j in cmds{
                        if j.name.contains(&argv[i]){
                            coms += 1;
                            if i >= argv.len()-1{
                                match check_input(j, &argv[i], None){
                                    Err(e) =>{
                                        return Err(e);
                                    },
                                    Ok((o, s)) => {
                                        instruction.insert(o, s);
                                    },
                                }
                            }
                            else{
                                match check_input(j, &argv[i], Some(&argv[i+1])){
                                    Err(e) =>{
                                        return Err(e);
                                    },
                                    Ok((o, s)) => {
                                        instruction.insert(o, s);
                                        i+=1;
                                    },
                                }
                            }
                        }
                    }
                }
                else{
                    return Err(Error::ArgumentNotFound(argv[i].clone()));
                }
                if coms <= 0 {
                    return Err(Error::ArgumentNotFound(argv[i].clone()));
                }
                else{
                    coms = 0;
                }
                i+=1;
            }
            Ok(instruction)
        }
    }
}
