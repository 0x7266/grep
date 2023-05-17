fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: Input = parse_config(&args);
    let content = std::fs::read_to_string(input.filename).expect("Error reading the file");
    println!("FILENAME: {}", content);
}

struct Input {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Input {
    Input {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}
