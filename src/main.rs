use std::fs;

fn main() {
    // 1) get current path
    let current_path = ".";
    // 2) get current path file object
    let directory = fs::read_dir(current_path).unwrap();
    // 3) get sub files
    let enums = directory.enumerate();
    let mut file_names = Vec::new();
    enums.for_each(|x| {
        let file_name = x.1.unwrap().file_name();
        file_names.push(file_name.into_string().unwrap());
    });
    // 4) etl handle
    // 5) output message
    for name in file_names {
        println!("{}", name);
    }
}
