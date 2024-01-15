use std::{env, fs};
struct Config{
    query:String,
    file_path:String
}
impl Config{
    fn new(configs: &[String]) -> Config{
        Config{
            query:configs[1].clone(),
            file_path:configs[2].clone()
        }
    }
}
fn extract_file_path_contents(file_path: &str) -> String{
    let contents = fs::read_to_string(file_path)
        .expect("Error reading the file at the specified path");
    contents
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let content = extract_file_path_contents(&config.file_path);
    println!("File text:\n{}", content);
}
