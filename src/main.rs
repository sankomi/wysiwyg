use std::{env, process};

fn main() {
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
    let value: String = args.collect::<Vec<String>>().join(" ");

    // print
    println!("{}: {:?}", command, value);
}
