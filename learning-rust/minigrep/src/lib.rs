use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) ->Result<Self, &'static str>{

        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Dindn't get a query string"),
        };
        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Dindn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self{query, file_path, ignore_case})

    }
}

pub fn run(config:Config)->Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result{
        println!("> {}", line);
    }

    Ok(())
    
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) ->Vec<&'a str>{

    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase() // return a String here
            .contains(&query))// we need to pass the string ref here because tolow return String
        .collect()

}

pub fn search<'a>(query: &str, content: &'a str) ->Vec<&'a str>{
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_search_for_word_should_find_line(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn when_search_for_other_word_should_find_line(){
        let query = "the";
        let content = "\
Rust:
safe, fast, productive.
toto the ccacc.
Pick three.";
        assert_eq!(vec!["toto the ccacc."], search(query, content));
    }
    #[test]
    fn when_search_for_other_word_should_find_all_lines(){
        let query = "to";
        let content = "\
Rust:to
safe, fast, productive.
toto the
Pick three.";
        assert_eq!(vec!["Rust:to", "toto the"], search(query, content));
    }

    #[test]
    fn when_search_insensitive_for_word_should_find_line(){
        let query = "rUst";
        let content = "\
Rust:
safe, fast, productive.
toto ThE ccacc.
Pick tHree.";
        assert_eq!(vec!["Rust:"], search_insensitive(query, content));
    }
}


