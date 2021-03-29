mod utils;

use colored::*;
use serde_json::Value;
use std::{
    env,
    fs::create_dir,
    fs::{self, remove_file},
    process,
};
use std::{fs::File, path::Path};
use std::{io::Write, time::Instant};

const __VERSION__: &str = "1.0.0";

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let command: &str = &args[1];

    match command {
        "add" => {
            println!("shc {} {}", __VERSION__, "add".bright_green());

            // 2 Possibilities : shc add cargo and shc add cru < cargo run
            // TODO: Handle Multiple Shortcuts shc add cargo,git

            if args.len() == 3 {
                let res: Value = utils::get_shortcut(&args[2]);
                let shortcuts = &res["shortcuts"].as_array().unwrap();

                for object in shortcuts.iter() {
                    let alias: &str = &object["alias"].as_str().unwrap();
                    let command: &str = &object["command"].as_str().unwrap();
                    generate_shortcut(alias, command);
                }

                let end = Instant::now();
                println!(
                    "Added {} {} in {:.2}s",
                    shortcuts.len().to_string().bright_green(),
                    "shortcuts",
                    (end - start).as_secs_f32()
                );
            } else {
            }
        }
        "remove" => {
            println!("shc {} {}", __VERSION__, "remove".bright_green());
            if args.len() == 3 {
                let res: Value = utils::get_shortcut(&args[2]);
                let shortcuts = &res["shortcuts"].as_array().unwrap();

                for object in shortcuts.iter() {
                    let alias: &str = &object["alias"].as_str().unwrap();
                    let command: &str = &object["command"].as_str().unwrap();
                    delete_shortcut(alias, command);
                }

                let end = Instant::now();
                println!(
                    "Removed {} {} in {:.2}s",
                    shortcuts.len().to_string().bright_green(),
                    "shortcuts",
                    (end - start).as_secs_f32()
                );
            }
        }
        _ => println!("Invalid Command!"),
    }
}

fn delete_shortcut(alias: &str, command: &str) {
    let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shc\\");
    let file_path = format!("{}{}.bat", bin, alias);
    let contents = fs::read_to_string(&file_path).unwrap();

    if contents.contains(&command) {
        remove_file(&file_path).unwrap_or_else(|error| {
            eprintln!("Failed To Delete Shortcut : {}", error);
            process::exit(1);
        });
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
                                .write_all(format!("@echo off\n{} %1", command).as_bytes())
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
                        .write_all(format!("@echo off\n{} %1", command).as_bytes())
                        .unwrap();
                }
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
