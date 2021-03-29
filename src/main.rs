mod utils;

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::{env, process};

const __VERSION__: &str = "1.0.0";

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let command: &str = &args[1];

    match command {
        "add" => {
            println!("shc {} {}", __VERSION__, "add".green().bold());

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
            println!("shc {} {}", __VERSION__, "remove".green().bold());
        }
        _ => println!("Invalid Command!"),
    }
    // get()

    // then we need to create shortcuts

    // then we need to print success
}

fn generate_shortcut(alias: &str, command: &str) {
    match env::consts::OS {
        "windows" => {}
        "macos" => {}
        "linux" => {}
        &_ => {
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}
