fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: Input = Input::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1) // exit without panic
    });
    run(input);
}

fn run(input: Input) {
    let content = std::fs::read_to_string(input.filename).expect("Error reading the file");
    println!("FILE CONTENT: {}", content);
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
