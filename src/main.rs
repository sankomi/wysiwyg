use std::io::{self, IsTerminal};
use std::{env, process};

fn main() {
    let mut piped = false;
    let mut value = String::new();

    // get value from stdin
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        stdin.read_line(&mut value)
            .unwrap_or_default();
        value = value.trim().to_string();
        piped = true;
    }

    // get args
    let mut args = env::args();

    // skip name
    args.next();

    // check command from second arg
    let second: String = match args.next() {
        Some(arg) => arg,
        None => {
            if !piped {
                eprintln!("value missing!");
                process::exit(1);
            }
            String::from("")
        }
    };
    let command = match second.as_str() {
        "-s" | "--set" => "set",
        "-g" | "--get" => "get",
        s => {
            if s.starts_with("-") {
                eprintln!("unknown command!");
                process::exit(1);
            }

            // add second arg to value if not command
            if !piped {
                value += s.trim();
            }
            "get"
        }
    };

    // add rest to value
    if !piped {
        let rest = args.collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();

        if value.is_empty() {
            value = rest;
        } else if !rest.is_empty() {
            value = value + " " + rest.as_str();
        }
    }

    // print command: value
    print!("{}: {:?}", command, value);
}
