use std::{env, error::Error, fs};

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.filename)?;
    let mut index = 1;
    let results = if input.case_sensitive {
        search(&input.query, &contents)
    } else {
        search_case_insensitive(&input.query, &contents)
    };
    println!("---------------------------------");
    println!("        RESULTS:        \n\n");
    for line in results {
        println!("{}- {}\n", index, line);
        index += 1;
    }
    println!("---------------------------------\n\n");
    Ok(())
}

pub struct Input {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Input {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
        )
    }
}
