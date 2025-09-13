use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::path::PathBuf;
use std::{env, process};

enum Command {
    SET,
    GET,
}

fn main() {
    let mut name: Option<String> = None;

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

    // get name from stdin
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        let mut read = String::new();
        stdin.read_line(&mut read).unwrap_or_default();
        name = Some(read.trim().to_string());
    }

    // get args
    let mut args = env::args();

    // skip name
    args.next();

    // check command from second arg
    let second: String = match args.next() {
        Some(arg) => arg,
        None => {
            if name.is_none() {
                eprintln!("name missing!");
                process::exit(1);
            } else {
                String::from("get")
            }
        }
    };
    let command = match second.as_str() {
        "-s" | "--set" => Command::SET,
        "-g" | "--get" => Command::GET,
        s => {
            if s.starts_with("-") {
                eprintln!("unknown command!");
                process::exit(1);
            }

            // use arg as name if not command
            if name.is_none() {
                name = Some(s.trim().to_string());
            }
            Command::GET
        }
    };

    // use third arg as name if not set
    if name.is_none() {
        name = args.next();
    }

    // unwrap name or die
    let name = match name {
        Some(n) => n,
        None => {
            eprintln!("name missing!");
            process::exit(1);
        }
    };

    // die if name is empty
    if name.is_empty() {
        eprintln!("name missing!");
        process::exit(1);
    }

    // run command
    match command {
        Command::SET => {
            // use rest of args as value
            let value = args.collect::<Vec<String>>().join(" ").trim().to_string();

            // die if nothing to set
            if value.is_empty() {
                eprintln!("value missing!");
                process::exit(1);
            }

            print!("set {}: {:?}", name, value);
        }
        Command::GET => {
            print!("get {}", name);
            let value = get_value(name);
            if let Err(_) = value  {
                eprintln!("value missing!");
                process::exit(1);
            }
            let value = value.unwrap();
            print!("{}", value);
        }
    }
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

fn get_value(name: String) -> Result<String, &'static str> {
    // open file
    let data_path = get_data_path()?;
    let file_path = data_path.join("test.txt");
    let file = File::open(file_path).map_err(|_| "failed to open file")?;

    // loop through lines
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let string = line.map_err(|_| "failed to read file")?;
        println!("{}", string);
    }

    // test
    Ok(String::from("value"))
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
