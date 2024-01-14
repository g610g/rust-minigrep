use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    let content = extract_file_path_contents(file_path);
    println!("File text:\n{}", content);
}

fn extract_file_path_contents(file_path: &str) -> String{
    let contents = fs::read_to_string(file_path)
        .expect("Error reading the file at the specified path");
    contents
}
