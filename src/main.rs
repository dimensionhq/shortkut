mod utils;

use colored::*;
use serde_json::Value;
use std::fs::{create_dir, read_to_string, remove_file, File};
use std::path::Path;
use std::process;
use std::{env, ffi::OsString, fs::read_dir, io::Read};
use std::{io::Write, time::Instant};
use termimad;

const __VERSION__: &str = "1.0.0";

// TODO: Allow Command File To Be A List Of Commands
//

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // Display Help Menu
        let help = format!(
            r#"{}
{} add - Add a shortcut
{} remove - Remove a shortcut
{} show - Show a shortcut pack
{} search - Search for a shortcut pack
{} export - Export a list of your current shortcuts"#,
            format!("shc {}", __VERSION__.bright_green()),
            "*".bright_magenta().bold(),
            "*".bright_magenta().bold(),
            "*".bright_magenta().bold(),
            "*".bright_magenta().bold(),
            "*".bright_magenta().bold(),
        );
        println!("{}", help);
    }

    if args.len() >= 3 {
        let command: &str = &args[1];

        match command {
            "add" => {
                println!("shc {} {}", __VERSION__, "add".bright_green().bold());

                // shc add cargo | shc add cargo,yarn
                if args.len() == 3 {
                    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
                    let mut installed: Vec<String> = vec![];

                    for arg in vec.iter() {
                        let res: Value = utils::get_shortcut(arg);
                        let shortcuts = &res["shortcuts"].as_array().unwrap();

                        for object in shortcuts.iter() {
                            let alias: &str = &object["alias"].as_str().unwrap();
                            let is_array = object["command"].is_array();

                            if !is_array {
                                let command = &object["command"].as_str().unwrap();
                                generate_shortcut(alias, command);
                            } else {
                                let commands = object["command"].as_array().unwrap();
                                generate_shortcut_multi(alias, commands);
                            }

                            installed.push(alias.to_string());
                        }
                    }

                    let display_string = installed.join(", ");
                    println!("Added Shortcuts: {}", display_string.bright_yellow());

                    let end = Instant::now();
                    println!(
                        "Added {} {} in {:.2}s",
                        installed.len().to_string().bright_green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else if args.len() == 2 {
                    println!(
                        "{}",
                        "Specify A Shortcut To Install\nUsage: shc add shorcut1,shortcut2"
                            .bright_yellow()
                    );
                } else if args.len() == 4 {
                    let alias = &args[2];
                    let command = &args[3].to_string().replace("\"", "");
                    generate_shortcut(alias, command);
                    let end = Instant::now();
                    println!(
                        "Added {} shortcut in {:.2}s",
                        1.to_string().bright_green(),
                        (end - start).as_secs_f32()
                    );
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".bright_yellow());
                }
            }
            "remove" => {
                println!("shc {} {}", __VERSION__, "remove".bright_green().bold());
                if args.len() == 3 {
                    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
                    let mut removed: Vec<String> = vec![];

                    for arg in vec.iter() {
                        let res: Value = utils::get_shortcut(arg);
                        let shortcuts = &res["shortcuts"].as_array().unwrap();

                        for object in shortcuts.iter() {
                            let alias: &str = &object["alias"].as_str().unwrap();
                            let is_array = object["command"].is_array();

                            if !is_array {
                                let command = &object["command"].as_str().unwrap();
                                generate_shortcut(alias, command);
                            } else {
                                let commands = object["command"].as_array().unwrap();
                                delete_shortcut_multi(alias, commands);
                            }
                            removed.push(alias.to_string());
                        }
                    }
                    let display_string = removed.join(", ");
                    println!("Removed Shortcuts: {}", display_string.bright_yellow());

                    let end = Instant::now();
                    println!(
                        "Removed {} {} in {:.2}s",
                        removed.len().to_string().bright_green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else if args.len() == 2 {
                    println!(
                        "{}",
                        "Specify A Shortcut To Remove\nUsage: shc remove shorcut1,shortcut2"
                            .bright_yellow()
                    );
                } else if args.len() == 4 {
                    let alias = &args[2];
                    let command = &args[3].to_string().replace("\"", "");
                    delete_shortcut(alias, command);
                    let end = Instant::now();
                    println!(
                        "Removed {} shortcut in {:.2}s",
                        1.to_string().bright_green(),
                        (end - start).as_secs_f32()
                    );
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".bright_yellow());
                }
            }
            "show" => {
                println!("shc {} {}", __VERSION__, "show".bright_green().bold());
                if args.len() == 3 {
                    let paths =
                        read_dir(format!("{}{}", env::var("USERPROFILE").unwrap(), "\\.shc"))
                            .unwrap();

                    for path in paths {
                        let file_name = format!(
                            "{}",
                            path.unwrap().file_name().to_os_string().to_str().unwrap()
                        );
                        let comp = format!(
                            "{}{}",
                            OsString::from(&args[2]).to_os_string().to_str().unwrap(),
                            ".bat"
                        );

                        if file_name == comp {
                            let mut command = String::new(); // Open file and display command
                            let mut file = File::open(format!(
                                "{}{}{}",
                                env::var("USERPROFILE").unwrap(),
                                "\\.shc\\",
                                &file_name
                            ))
                            .unwrap();
                            file.read_to_string(&mut command).unwrap();
                            command = command
                                .replace("@echo off", "")
                                .replace("%*", "")
                                .replace("\n", "");
                            println!("{} : {}", &args[2].cyan(), command.bright_yellow());
                            process::exit(0);
                        }
                    }

                    let res: Value = utils::get_shortcut(&args[2]);
                    let shortcuts = &res["shortcuts"].as_array().unwrap();

                    for object in shortcuts.iter() {
                        // let alias: &str = &object["alias"].as_str().unwrap();
                        let is_array = object["command"].is_array();

                        if !is_array {
                            let description = &object["description"].as_str().unwrap();

                            println!("{}", termimad::inline(description));
                        } else {
                            let description = &object["description"].as_str().unwrap();
                            // println!("{}", description.bright_white());
                            println!("{}", termimad::inline(description));
                        }
                    }

                    let end = Instant::now();
                    println!(
                        "Found {} {} in {:.2}s",
                        shortcuts.len().to_string().bright_green(),
                        "shortcuts",
                        (end - start).as_secs_f32()
                    );
                } else if args.len() == 4 {
                } else {
                    println!("{}", "shc Recieved Unexpected Arguments".bright_yellow());
                }
            }
            "search" => {
                let approx = &args[2];
                let response = utils::send_search_query(String::from(approx));
                if response != "" {
                    if &response == approx {
                        println!("{}", response.bold().bright_green());
                    } else {
                        println!("{}", response.bold().bright_yellow());
                    }
                } else {
                    println!(
                        "{} {} {}",
                        0.to_string().bold().bright_red(),
                        "Matches Found For",
                        &approx.bold().bright_yellow().underline(),
                    );
                    process::exit(1);
                }

                let end = Instant::now();
                println!(
                    "Found {} {} in {:.2}s",
                    "1".to_string().bright_green(),
                    "shortcut",
                    (end - start).as_secs_f32()
                );
            }
            _ => println!("Invalid Command!"),
        }
    }
}

fn delete_shortcut_multi(alias: &str, command: &Vec<Value>) {
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
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}

fn generate_shortcut_multi(alias: &str, command: &Vec<Value>) {
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
            // alias alias='command'
        }
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}
