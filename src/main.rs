mod utils;

use colored::*;
use serde_json::Value;
use std::env;
use std::fs::{create_dir, read_to_string, remove_file, File};
use std::path::Path;
use std::process;
use std::{io::Write, time::Instant};

const __VERSION__: &str = "1.0.0";

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // Display Help Menu
        let help = format!(
            r#"
{}
{} add - Add a shortcut
{} remove - Remove a shortcut
{} show - Show a shortcut
        "#,
            format!("shc {}", __VERSION__),
            "*".magenta(),
            "*".magenta(),
            "*".magenta(),
        );
        println!("{}", help);
    }

    if args.len() >= 3 {
        let command: &str = &args[1];

        match command {
            "add" => {
                println!("shc {} {}", __VERSION__, "add".green());

                // shc add cargo | shc add cargo,yarn
                if args.len() == 3 {
                    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
                    let mut installed: Vec<String> = vec![];

                    for arg in vec.iter() {
                        let res: Value = utils::get_shortcut(arg);
                        let shortcuts = &res["shortcuts"].as_array().unwrap();

                        for object in shortcuts.iter() {
                            let alias: &str = &object["alias"].as_str().unwrap();
                            let command: &str = &object["command"].as_str().unwrap();
                            generate_shortcut(alias, command);
                            installed.push(alias.to_string());
                        }
                    }

                    let display_string = installed.join(", ");
                    println!("Added Shortcuts: {}", display_string.yellow());

                    let end = Instant::now();
                    println!(
                        "Added {} {} in {:.2}s",
                        installed.len().to_string().green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else if args.len() == 2 {
                    println!(
                        "{}",
                        "Specify A Shortcut To Install\nUsage: shc add shorcut1,shortcut2".yellow()
                    );
                } else if args.len() == 4 {
                    let alias = &args[2];
                    let command = &args[3].to_string().replace("\"", "");
                    generate_shortcut(alias, command);
                    let end = Instant::now();
                    println!(
                        "Added {} shortcut in {:.2}s",
                        1.to_string().green(),
                        (end - start).as_secs_f32()
                    );
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".yellow());
                }
            }
            "remove" => {
                println!("shc {} {}", __VERSION__, "remove".green());
                if args.len() == 3 {
                    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
                    let mut removed: Vec<String> = vec![];

                    for arg in vec.iter() {
                        let res: Value = utils::get_shortcut(arg);
                        let shortcuts = &res["shortcuts"].as_array().unwrap();

                        for object in shortcuts.iter() {
                            let alias: &str = &object["alias"].as_str().unwrap();
                            let command: &str = &object["command"].as_str().unwrap();
                            delete_shortcut(alias, command);
                            removed.push(alias.to_string());
                        }
                    }

                    let display_string = removed.join(", ");
                    println!("Removed Shortcuts: {}", display_string.yellow());

                    let end = Instant::now();
                    println!(
                        "Removed {} {} in {:.2}s",
                        removed.len().to_string().green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else if args.len() == 2 {
                    println!(
                        "{}",
                        "Specify A Shortcut To Remove\nUsage: shc remove shorcut1,shortcut2"
                            .yellow()
                    );
                } else if args.len() == 4 {
                    let alias = &args[2];
                    let command = &args[3].to_string().replace("\"", "");
                    delete_shortcut(alias, command);
                    let end = Instant::now();
                    println!(
                        "Removed {} shortcut in {:.2}s",
                        1.to_string().green(),
                        (end - start).as_secs_f32()
                    );
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".yellow());
                }
            }
            "show" => {
                println!("shc {} {}", __VERSION__, "show".green());
                if args.len() == 3 {
                    let res: Value = utils::get_shortcut(&args[2]);
                    let shortcuts = &res["shortcuts"].as_array().unwrap();

                    for object in shortcuts.iter() {
                        let alias: &str = &object["alias"].as_str().unwrap();
                        let command: &str = &object["command"].as_str().unwrap();
                        println!("{} : {}", alias.cyan(), command.yellow())
                    }

                    let end = Instant::now();
                    println!(
                        "Found {} {} in {:.2}s",
                        shortcuts.len().to_string().green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".yellow());
                }
            }
            _ => println!("Invalid Command!"),
        }
    }
}

fn delete_shortcut(alias: &str, command: &str) {
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
            println!("{}", "OS Not Supported!".yellow());
            process::exit(1);
        }
    }
}

fn generate_shortcut(alias: &str, command: &str) {
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
                            ".shc".red(),
                            err.to_string().yellow()
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
        "macos" => {}
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".yellow());
            process::exit(1);
        }
    }
}
