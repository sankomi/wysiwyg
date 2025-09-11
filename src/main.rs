use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::{env, process};

fn main() {
    let mut piped = false;
    let mut value = String::new();

    // get data path and test write
    let data_path = match get_data_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if let Err(e) = save_text(&data_path) {
        eprintln!("{}", e);
        process::exit(1);
    }

    // get value from stdin
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        stdin.read_line(&mut value).unwrap_or_default();
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
        let rest = args.collect::<Vec<String>>().join(" ").trim().to_string();

        if value.is_empty() {
            value = rest;
        } else if !rest.is_empty() {
            value = value + " " + rest.as_str();
        }
    }

    // print command: value
    print!("{}: {:?}", command, value);
}

fn get_data_path() -> Result<PathBuf, &'static str> {
    // return $XDG_DATA_HOME if possible
    let data_home = env::var_os("XDG_DATA_HOME");
    if let Some(v) = data_home {
        return Ok(PathBuf::from(v).join("wysiwyg"));
    }

    // check if $HOME exists, else return error
    let home = env::var_os("HOME");
    if home.is_none() {
        return Err("$HOME missing!");
    }
    let home = home.unwrap();
    if home.is_empty() {
        return Err("$HOME missing!");
    }

    Ok(PathBuf::from(home).join(".local/share/wysiwyg"))
}

fn save_text(path: &PathBuf) -> Result<(), &'static str> {
    // create dir
    if let Err(_) = fs::create_dir_all(path) {
        return Err("failed to create data path!");
    }

    // open file
    let file_path = path.join("test.txt");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path);
    if let Err(_) = file {
        return Err("failed to open data file!");
    }
    let mut file = file.unwrap();

    // write things
    if let Err(_) = writeln!(file, "test!") {
        return Err("failed to write data file!");
    }

    Ok(())
}
