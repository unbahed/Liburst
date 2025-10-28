mod file_system;
mod arg;

use std::path::PathBuf;

fn main() {
    let file = "test_file".to_string();

    let tmp = file_system::read_lines(&file);
    println!("{:?}", tmp);
    let tmp = file_system::delete_char(2, &file);
    println!("{:?}", tmp);
    let tmp = file_system::read_lines(&file);
    println!("{:?}", tmp);

}
