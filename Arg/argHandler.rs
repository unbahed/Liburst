use std::env::{Args, args};
use std::any::{type_name, type_name_of_val};
use std::collections::HashMap;
use std::fmt;
use crate::lib::Arg::error::Error;
use crate::lib::Arg::args::{Argm, InputType, Dep};


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

/*
fn check_dep(){
}
*/

fn check_input(arg: &Argm, argv: &Vec<Option<&String>>) -> Result<(Argm, Option<String>), Error>{
    match &arg.input{
        InputType::None =>{
            if argv[1] == None{
                return Ok((arg.clone(), None));
            }
            else{
                return Err(Error::InputNotNeeded(argv[0].unwrap().clone()));
            }
        },
        InputType::Open(t) => {
            match argv[1].ok_or(Error::InputNotGiven(argv[0].unwrap().clone())){
                Ok(s) => {
                    if is_command(s){
                        return Err(Error::InputNotFound(arg.name[1].clone(), s.clone()));
                    }
                    else{
                        return Ok((arg.clone(), Some(s.clone())));
                    }
                },
                Err(e) => {
                    return Err(Error::InputNotGiven(argv[0].unwrap().clone()));
                },
            }
        },
        InputType::Strict(t, l) => {
            match argv[1].ok_or(Error::InputNotGiven(argv[0].unwrap().clone())){
                Ok(s)=>{
                    if l.clone().unwrap().contains(s){
                        return Ok((arg.clone(), Some(s.clone())));
                    }
                    else{
                        return Err(Error::InputNotFound(arg.name[1].clone(), s.clone()));
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
                                match check_input(j, &vec![Some(&argv[i]), None]){
                                    Err(e) =>{
                                        return Err(e);
                                    },
                                    Ok((o, s)) => {
                                        instruction.insert(o, s);
                                    },
                                }
                            }
                            else{
                                match check_input(j, &vec![Some(&argv[i]), Some(&argv[i+1])]){
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
