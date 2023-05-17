use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Input = Input::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1) // exit without panic
    });
    if let Err(err) = run(input) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}

fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(input.filename)?;
    println!("FILE CONTENT: {}", content);

    Ok(())
}

struct Input {
    query: String,
    filename: String,
}

impl Input {
    fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Input {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
