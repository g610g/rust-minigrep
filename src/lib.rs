use std::{fs,env};
use std::error::Error;

pub struct Config{
    query:String,
    file_path:String,
    ignore_case:bool
}
impl Config{
    pub fn build(configs: &[String]) -> Result<Config, &'static str>{
       if configs.len() < 3{
        return Err("not enough arguments");
       }
       let query = configs[1].clone();
       let file_path = configs[2].clone();
       let ignore_case = env::var("IGNORE_CASE").is_ok();
       Ok(Config{query, file_path, ignore_case})
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.file_path)?;
    let result  = if config.ignore_case{
        insensitive_search(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}
pub fn insensitive_search<'a>(query:&str, contents:&'a str)-> Vec<&'a str>{
    let mut result: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in contents.lines(){
            if line.to_lowercase().contains(&query){
                result.push(line);
            }
    }
    return result;
}
pub fn search<'a >(query:&str, contents:&'a str)-> Vec<&'a str>{
    let mut result: Vec<&str> = vec![];
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        } 
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn search_test_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three:";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
    #[test]
    fn search_test_insensitive(){
        let query = "Io";
        let contents = "\
gio gonzales
standard IO library
just a dummy text i cant think of anything";
        assert_eq!(vec!["gio gonzales", "standard IO library"], insensitive_search(query, contents));
}
} 
