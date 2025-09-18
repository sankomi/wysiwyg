use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::path::PathBuf;
use std::{env, process};

enum Command {
    SET,
    GET,
}

fn main() {
    let mut name: Option<String> = None;

    // get name from stdin
    let mut names: Option<Vec<String>> = None;
    let stdin = io::stdin();
    let terminal = stdin.is_terminal();
    if !terminal {
        let mut strings = vec![];

        for line in stdin.lock().lines() {
            if let Ok(string) = line {
                // die if name is empty or not alpha numeric
                if string.is_empty() {
                    die("name missing!");
                } else if !string.chars().all(char::is_alphanumeric) {
                    die("name invalid!");
                }

                strings.push(string);
            }
        }

        if strings.len() > 0 {
            names = Some(strings);
        } else {
            die("name missing!");
        }
    }

    // get args
    let mut args = env::args();

    // skip name
    args.next();

    // check command from second arg
    let second: String = match args.next() {
        Some(arg) => arg,
        None => {
            if terminal {
                die("name missing!")
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
                die("unknown command!");
            }

            // use arg as name if terminal
            if terminal {
                name = Some(s.trim().to_string());
            }
            Command::GET
        }
    };

    if terminal {
        // use third arg as name if not set
        if name.is_none() {
            name = args.next();
        }
    }

    // run command
    match command {
        Command::SET => {
            // get single name or die
            name = if terminal {
                name
            } else {
                let names = names.unwrap();
                if names.len() != 1 {
                    die("multiple names!");
                }

                names.first().cloned()
            };

            // unwrap name or die
            let name = match name {
                Some(n) => n,
                None => die("name missing!"),
            };

            check_name(&name);

            // use rest of args as value
            let value = args.collect::<Vec<String>>().join(" ").trim().to_string();

            // die if nothing to set
            if value.is_empty() {
                die("value missing!");
            }

            let saved = set_value(name, &value);
            if let Err(e) = saved {
                die(e);
            }
            end(&value);
        }
        Command::GET => {
            // get all names
            names = if terminal {
                vec![name.unwrap()].into()
            } else {
                names
            };
            let names = names.unwrap();

            let mut values: Vec<String> = vec![];
            for name in names {
                check_name(&name);

                match get_value(name) {
                    Ok(value) => values.push(value),
                    Err(e) => {
                        die(e);
                    }
                }
            }
            for value in values {
                end(&value);
            }
        }
    }
}

fn die(message: &str) -> String {
    let exit_code = env::var_os("WYSIWYG_EXIT_CODE").is_some();
    let error_message = env::var_os("WYSIWYG_ERROR_MESSAGE").is_some();

    if error_message {
        eprintln!("{}", message);
    } else {
        eprintln!("?");
    }

    if exit_code {
        process::exit(1);
    } else {
        process::exit(0);
    }
}
fn end(value: &str) {
    println!("{}", value);
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

    // create dir
    let path = PathBuf::from(home).join(".local/share/wysiwyg");
    if fs::create_dir_all(&path).is_err() {
        return Err("failed to create data path!");
    }

    Ok(path)
}

fn get_value(name: String) -> Result<String, &'static str> {
    // open file
    let data_path = get_data_path()?;
    let file_path = data_path.join("data");
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(file_path)
        .map_err(|_| "failed to open file!")?;

    // loop through lines
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let string = line.map_err(|_| "failed to read file!")?;
        let (key, value) = match string.split_once(" ") {
            Some((key, value)) => (key, value),
            None => continue,
        };
        if key == name {
            return Ok(value.to_string());
        }
    }

    Err("value missing!")
}

fn set_value(name: String, value: &str) -> Result<(), &'static str> {
    // open file
    let data_path = get_data_path()?;
    let file_path = data_path.join("data");
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&file_path)
        .map_err(|_| "failed to open file!")?;

    // get all lines except matching one(s)
    let mut lines: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // skip unreadable lines
        if line.is_err() {
            continue;
        }

        // split line and skip unsplittable lines
        let string = line.unwrap();
        let key = match string.split_once(" ") {
            Some((key, _)) => key,
            None => continue,
        };

        // add to lines if name does not match
        if key != name {
            lines.push(string);
        }
    }

    // add name value pair
    lines.push(name + " " + &value);

    // write data file and replace
    let temp_path = data_path.join("temp_data");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temp_path)
        .map_err(|_| "failed to write data file!")?;
    file.write_all(lines.join("\n").as_bytes())
        .map_err(|_| "failed to write data file!")?;
    fs::rename(temp_path, file_path).map_err(|_| "failed to update data file!")?;

    Ok(())
}

fn check_name(name: &str) {
    // die if name is empty or not alpha numeric
    if name.is_empty() {
        die("name missing!");
    } else if !name.chars().all(char::is_alphanumeric) {
        die("name invalid!");
    }
}
