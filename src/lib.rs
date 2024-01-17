use std::{fs};
use std::error::Error;

pub struct Config{
    query:String,
    file_path:String
}
impl Config{
    pub fn build(configs: &[String]) -> Result<Config, &'static str>{
       if configs.len() < 3{
        return Err("not enough arguments");
       }
       let query = configs[1].clone();
       let file_path = configs[2].clone();
       Ok(Config{query, file_path})
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.file_path)?;
    Ok(())
}
pub fn search<'a >(query:&str, contents:&'a str)-> Vec<&'a str>{
    return vec![];
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn search_test(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive
        Pick three:";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
} 
