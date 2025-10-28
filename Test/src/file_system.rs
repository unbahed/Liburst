use std::path::Path;
use std::path::PathBuf;
use std::fs::{read, write};

#[derive(Debug)]
pub enum Error{
    Silent,
    UnableToRead(String),
    UnableToWrite(String),
    NotFound(String),
    NotFile(String),
    OutOfBoundN(usize),
}

fn check_file(given_path: &String) -> Result<Vec<u8>, Error>{
    let file_path = PathBuf::from(&given_path);
    match file_path.exists(){
        true => {
            match file_path.is_file(){
                true => {
                    match read(file_path){
                        Ok(content) =>{
                            return Ok(content);
                        },
                        Err(_) => Err(Error::UnableToRead(given_path.clone()))
                    }
                },
                false => Err(Error::NotFile(given_path.clone()))
            }
        },
        false => Err(Error::NotFound(given_path.clone()))
    }
}

pub fn read_lines(given_path: &String) -> Result<Vec<String>, Error>{
    match check_file(&given_path){
        Ok(file_content) => {
            let mut content_lines: Vec<String> = Vec::new();
            let mut chr = "".to_string();
            let new_line = '\n' as u8;
            for (index, val) in file_content.iter().enumerate(){
                if val == &new_line{
                    content_lines.push(chr);
                    chr = "".to_string();
                }
                else if index >= file_content.len() -1 {
                    content_lines.push(chr);
                    chr = "".to_string();
                }
                else{
                    chr += &char::from(*val).to_string();
                }
            }
            Ok(content_lines)
        },
        Err(error) => Err(error)
    }
}

pub fn line_count(given_path: &String) -> Result<i32, Error>{
    match check_file(&given_path){
        Ok(file_content) => {
            let mut counter: i32 = 0;
            for i in file_content {
                if i == '\n' as u8{
                    counter += 1;
                }
            }
            return Ok(counter);
        },
        Err(error) => {
            return Err(error);
        }
    }
}

pub fn read_n_line(n: usize, given_path: &String) -> Result<String, Error>{
    match read_lines(given_path){
        Ok(content_lines) => {
            if n < content_lines.len(){
                return Ok(content_lines[n].clone());
            }
            else{
                return Err(Error::OutOfBoundN(n));
            }
        },
        Err(error) => {
                return Err(error);
        },
    }
}

pub fn read_n_char(n: usize, given_path: &String) -> Result<char, Error>{
    match check_file(&given_path){
        Ok(content) => {
            if n < content.len(){
                Ok(char::from(content[n]))
            }
            else{
                Err(Error::OutOfBoundN(n))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn read_all_char(given_path: &String) -> Result<Vec<char>, Error>{
    match check_file(&given_path){
        Ok(content) => {
            let mut char_list: Vec<char> = Vec::new();
            for i in content{
                char_list.push(char::from(i));
            }
            return Ok(char_list);
        },
        Err(error) => {
            return Err(error);
        },
    }
}

pub fn read_bytes(given_path: &String) -> Result<Vec<u8>, Error>{
    match check_file(&given_path){
        Ok(content) => {
            return Ok(content);
        },
        Err(error) => {
            return Err(error);
        }
    }
}

pub fn read_n_byte(n: usize, given_path: &String) -> Result<u8, Error>{
    match check_file(&given_path){
        Ok(content) => {
            if n < content.len(){
                Ok(content[n])
            }
            else{
                Err(Error::OutOfBoundN(n))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn read_byte_lines(given_path: &String) -> Result<Vec<Vec<u8>>, Error>{
    match check_file(&given_path){
        Ok(content) => {
            let mut content_lines: Vec<Vec<u8>> = Vec::new();
            let mut chr: Vec<u8> = Vec::new();
            let new_line = '\n' as u8;
            for (index, byte) in content.iter().enumerate(){
                if byte == &new_line {
                    content_lines.push(chr);
                    chr = Vec::new();
                }
                else if index >= content.len() -1 {
                    chr.push(*byte);
                    content_lines.push(chr);
                    chr = Vec::new();
                }
                else{
                    chr.push(*byte);
                }
            }
            Ok(content_lines)
        },
        Err(error) => Err(error)
    }
}


pub fn appendln(new_bytes: Vec<u8>, given_path: &String) -> Result<(), Error>{
    match check_file(&given_path){
        Ok(mut content) => {
            content.push('\n' as u8);
            for i in new_bytes{
                content.push(i);
            }
            match write(PathBuf::from(&given_path), content){
                Ok(_) => Ok(()),
                Err(_) => Err(Error::UnableToWrite(given_path.clone()))
            }
        },
        Err(error) => Err(error),
    }
}

pub fn append(new_bytes: Vec<u8>, given_path: &String) -> Result<(), Error>{
    match check_file(&given_path){
        Ok(mut content) => {
            for i in new_bytes{
                content.push(i);
            }
            match write(PathBuf::from(&given_path), content){
                Ok(_) => Ok(()),
                Err(_) => Err(Error::UnableToWrite(given_path.clone()))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn overwrite(new_bytes: Vec<u8>, given_path: &String) -> Result<(), Error>{
    match check_file(&given_path){
        Ok(_) => {
            match write(&given_path, new_bytes){
                Ok(_) => Ok(()),
                Err(_) => Err(Error::UnableToWrite(given_path.clone()))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn delete_line(n: usize, given_path: &String) -> Result<(), Error>{
    match read_byte_lines(given_path){
        Ok(mut line_list) => {
            if n < line_list.len(){
                line_list.remove(n);
                let mut content: Vec<u8> = Vec::new();
                let new_line = '\n' as u8;
                for i in line_list{
                    for y in i{
                        content.push(y);
                    }
                    content.push(new_line);
                }
                match overwrite(content, given_path){
                    Ok(_) => Ok(()),
                    Err(error) => Err(error)
                }
            }
            else{
                Err(Error::OutOfBoundN(n))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn delete_char(n: usize, given_path: &String) -> Result<(), Error>{
    match read_bytes(given_path){
        Ok(mut bytes) => {
            if n < bytes.len(){
                bytes.remove(n);
                match overwrite(bytes, given_path){
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            else{
                Err(Error::OutOfBoundN(n))
            }
        },
        Err(error) => Err(error)
    }
}

