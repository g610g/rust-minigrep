use std::{fs,env};
use std::error::Error;

pub struct Config{
    query:String,
    file_path:String,
    ignore_case:bool
}
impl Config{
    pub fn build(mut configs: impl Iterator<Item = String>) -> Result<Config, &'static str>{
       configs.next();
       let query = match configs.next(){
            Some(arg) => arg,
            None => return Err("Did not get the query string")
       };
       let file_path = match configs.next() {
           Some(file_path) => file_path,
           None => return Err("Did not get the file path")
       };
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
    let query = query.to_lowercase();
    let result = contents.lines()
            .filter(|line|line.to_lowercase().contains(&query))
            .collect();
    result
}
pub fn search<'a >(query:&str, contents:&'a str)-> Vec<&'a str>{
    let result = contents.lines()
                                    .filter(|line| line.contains(query))
                                    .collect();
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
