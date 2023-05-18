use std::{error::Error, fs};

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(input.filename)?;
    println!("FILE CONTENT: {}", content);

    Ok(())
}

pub struct Input {
    pub query: String,
    pub filename: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Input {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "three";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Pick three."], search(query, contents))
    }
}
