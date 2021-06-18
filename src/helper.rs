use colored::Colorize;
use serde_json::Value;
use std::env;
use std::fs::{create_dir, read_to_string, remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process;

pub fn delete_shortcut_multi(alias: &str, command: &Vec<Value>) {
    match env::consts::OS {
        "windows" => {
            let command_string: &String = &command
                .iter()
                .map(|value| format!("{}\n", value.to_string().replace("\"", "")))
                .collect::<String>();

            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");
            let file_path = format!("{}{}.bat", bin, alias);
            let contents = read_to_string(&file_path).unwrap_or_else(|_| {
                return String::new();
            });

            if contents.contains(&command_string.as_str()) {
                remove_file(&file_path).unwrap_or_else(|error| {
                    eprintln!("Failed To Delete Shortcut : {}", error);
                    process::exit(1);
                });
            }
        }
        "macos" => {}
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}

pub fn delete_shortcut(alias: &str, command: &str, shell: String) {
    match env::consts::OS {
        "windows" => {
            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");
            let file_path = format!("{}{}.bat", bin, alias);
            let contents = read_to_string(&file_path).unwrap_or_else(|_| {
                return String::new();
            });

            if contents.contains(&command) {
                remove_file(&file_path).unwrap_or_else(|error| {
                    eprintln!("Failed To Delete Shortcut : {}", error);
                    process::exit(1);
                });
            }
        }
        "macos" => {}
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}

pub fn generate_shortcut_multi(alias: &str, command: &Vec<Value>, shell: String) {
    match env::consts::OS {
        "windows" => {
            let command_string: &String = &command
                .iter()
                .map(|value| format!("{}\n", value.to_string().replace("\"", "")))
                .collect::<String>();

            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");

            let file = Path::new(&bin);

            if !file.exists() {
                match create_dir(&bin) {
                    Ok(_) => {
                        let location: String = format!("{}{}.bat", &bin, &alias);
                        let path = Path::new(location.as_str());
                        if !path.exists() {
                            let mut batch = File::create(location).expect("Failed To Create File");
                            batch
                                .write_all(format!("@echo off\n{}", command_string).as_bytes())
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "Failed To Create {} : {}",
                            ".shc".bright_red(),
                            err.to_string().bright_yellow()
                        );
                        process::exit(1);
                    }
                };
            } else {
                let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");
                let location: String = format!("{}{}.bat", &bin, &alias);
                let path = Path::new(location.as_str());
                if !path.exists() {
                    let mut batch = File::create(location).expect("Failed To Create File");
                    batch
                        .write_all(format!("@echo off\n{}", command_string).as_bytes())
                        .unwrap();
                }
            }
        }
        &_ => {
            let commands = command
                .into_iter()
                .map(|v| {
                    v.to_string()
                        .replace("%", "$")
                        .replace("$*", "\"$@\"")
                        .replace("\"", "")
                })
                .collect::<Vec<String>>();

            let mut location = String::new();

            match shell.as_str() {
                "/bin/bash" => {
                    location = format!("{}/.bashrc", home::home_dir().unwrap().display());
                }
                "/bin/zsh" => {
                    location = format!("{}/.zshrc", home::home_dir().unwrap().display());
                }
                _ => {}
            }

            let path = Path::new(location.as_str());

            if path.exists() {
                match OpenOptions::new().read(true).append(true).open(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let command = commands.join("\n    ");
                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!("shc must be run with {} permissions", "sudo".underline())
                                .bright_red()
                                .bold()
                        );
                        println!("{}", err);
                    }
                };
            }
        }
    }
}

pub fn generate_shortcut(alias: &str, command: &str, shell: String) {
    match env::consts::OS {
        "windows" => {
            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");

            let file = Path::new(&bin);

            if !file.exists() {
                match create_dir(&bin) {
                    Ok(_) => {
                        let location: String = format!("{}{}.bat", &bin, &alias);
                        let path = Path::new(location.as_str());
                        if !path.exists() {
                            let mut batch = File::create(location).expect("Failed To Create File");
                            batch
                                .write_all(format!("@echo off\n{} %*", command).as_bytes())
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "Failed To Create {} : {}",
                            ".shc".bright_red(),
                            err.to_string().bright_yellow()
                        );
                        process::exit(1);
                    }
                };
            } else {
                let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");
                let location: String = format!("{}{}.bat", &bin, &alias);
                let path = Path::new(location.as_str());
                if !path.exists() {
                    let mut batch = File::create(location).expect("Failed To Create File");
                    batch
                        .write_all(format!("@echo off\n{} %*", command).as_bytes())
                        .unwrap();
                }
            }
        }
        &_ => {
            let mut location = String::new();

            match shell.as_str() {
                "/bin/bash" => {
                    location = format!("{}/.bashrc", home::home_dir().unwrap().display());
                }
                "/bin/zsh" => {
                    location = format!("{}/.zshrc", home::home_dir().unwrap().display());
                }
                _ => {}
            }

            let path = Path::new(location.as_str());

            if path.exists() {
                match OpenOptions::new().read(true).append(true).open(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!("shc must be run with {} permissions", "sudo".underline())
                                .bright_red()
                                .bold()
                        );
                        println!("{}", err);
                    }
                };
            } else {
                match File::create(location) {
                    Ok(mut file) => {
                        file.write_all(format!("alias {}='{}'\n", alias, command).as_bytes())
                            .unwrap();
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!("shc must be run with {} permissions", "sudo".underline())
                                .bright_red()
                                .bold()
                        );
                        println!("{}", err);
                    }
                }
            }
        }
    }
}

pub fn initialize() {
    let directory = format!(r"{}/.shc", home::home_dir().unwrap().display());

    let path = Path::new(directory.as_str());

    if !path.exists() {
        create_dir(directory).unwrap();
    }
}
