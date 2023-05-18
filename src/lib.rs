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
