use std::{env, fs};
struct Config{
    query:String,
    file_path:String
}
fn extract_file_path_contents(file_path: &str) -> String{
    let contents = fs::read_to_string(file_path)
        .expect("Error reading the file at the specified path");
    contents
}
fn parse_config(configs:&[String])-> Config{
    let query = configs[1].clone();
    let file_path = configs[2].clone();
    Config{query, file_path}
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let content = extract_file_path_contents(&config.file_path);
    println!("File text:\n{}", content);
}
