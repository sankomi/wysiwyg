use std::io::{self, IsTerminal};
use std::{env, process};

fn main() {
    let mut value = String::new();

    // get value from stdin
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        stdin.read_line(&mut value)
            .unwrap_or_default();
        value = value.trim().to_string();
    }

    // get args
    let mut args = env::args();

    // skip name
    args.next();

    // second arg as command (set, get)
    let command: String = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("command missing!");
            process::exit(1);
        }
    };
    if !["set", "get"].contains(&command.as_str()) {
        eprintln!("invalid command!");
        process::exit(1);
    }

    // rest as value
    if value.is_empty() {
        value = args.collect::<Vec<String>>().join(" ");
    }

    // print value
    print!("{}", value);
}
