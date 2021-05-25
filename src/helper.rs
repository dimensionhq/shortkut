use colored::Colorize;
use serde_json::Value;
use std::env;
use std::fs::{create_dir, read_dir, read_to_string, remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

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

pub fn delete_shortcut(alias: &str, command: &str) {
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

pub fn generate_shortcut_multi(alias: &str, command: &Vec<Value>) {
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
        "macos" => {
            // alias alias='command'
        }
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}

pub fn generate_shortcut(alias: &str, command: &str) {
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
        "macos" => {
            let location: String = String::from("~/.bashrc");

            let path = Path::new(location.as_str());

            if path.exists() {
                match OpenOptions::new().append(true).open(location) {
                    Ok(mut file) => {
                        // Write Alias To File (Append)
                        let write_string = format!("alias {}='{}'", alias, command);
                        file.write_all(write_string.as_bytes()).unwrap();
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
                        file.write_all(format!("alias {}='{}'", alias, command).as_bytes())
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

            Command::new("zsh")
                .arg("-c")
                .arg("'source ~/.bashrc'")
                .spawn()
                .unwrap();
        }
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}

#[allow(dead_code)]
fn export_shortcuts() {
    File::create("shortcuts.shk").unwrap();

    match env::consts::OS {
        "windows" => {
            let list = read_dir(format!(r"{}\.shc", env!("USERPROFILE"))).unwrap();
            let mut files: Vec<String> = vec![];
            for f in list {
                let f = f.unwrap();
                files.push(f.path().into_os_string().into_string().unwrap());
            }
        }
        _ => {}
    }
}

pub fn initialize() {
    let directory = format!(r"{}\.shc", env!("USERPROFILE"));
    let path = Path::new(directory.as_str());
    if !path.exists() {
        create_dir(directory).unwrap();
    }
}
