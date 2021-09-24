use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in &results {
        println!("{}", line); //to stdout
    }

    //println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get word to search"),
        };
        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //is_err - true if Err(_) else i.e Ok() false

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();
    //for line in contents.lines() { -- split with linebreak
    //    if line.contains(query) {
    //      results.push(line);
    //    }
    //}
    //results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();
    //let query = query.to_lowercase(); - preprocess
    //for line in contents.lines() {
    //  if line.to_lowercase().contains(&query) {
    //    results.push(line);
    //  }
    //}
    //results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//TDD (Test Driven Development) --- test first
//  Write a test that fails and run it to make sure it fails for the reason you expect.
//  Write or modify just enough code to make the new test pass.
//  Refactor the code you just added or changed and make sure the tests continue to pass.
//  Repeat from step 1!

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
