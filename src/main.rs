use minigrep::Input;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Input = Input::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err); // now if an error occurs the output will be blank (eg. cargo run > output.txt)
        process::exit(1) // exit without panic
    });
    if let Err(err) = minigrep::run(input) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
