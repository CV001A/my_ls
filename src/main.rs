use std::fs::File;
use std::fs;

fn main() {
    // 1) get current path
    let current_path = ".";
    // 2) get current path file object
    let directory = fs::read_dir(current_path).unwrap();
    // 3) get sub files
    let enums = directory.enumerate();
    let mut file_names = Vec::new();
    enums.for_each(|x| file_names.push(x.1.unwrap().name));
    // 4) etl handle
    // 5) output message
    println!("{}", file_names);

}
