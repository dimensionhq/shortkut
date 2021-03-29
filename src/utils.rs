use colored::*;
use reqwest::blocking;
use serde_json::Value;
use std::process;

pub fn get_shortcut(name: &str) -> Value {
    let mut res: String = String::new();

    match blocking::get(format!(
        "https://raw.githubusercontent.com/XtremeDevX/shc/master/shortcuts/{}.json",
        name
    )) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                // Response Code Is 200 OK
                match response.text() {
                    Ok(text) => {
                        res = text;
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            "Failed To Parse Response:".to_string().yellow(),
                            e.to_string().bright_red()
                        );
                        process::exit(1);
                    }
                }
            } else {
                println!(
                    "{} is not a valid shortcut pack.",
                    name.to_string().bright_magenta()
                );
                process::exit(1);
            }
        }
        Err(err) => println!("Failed To Request {}.json => {}", name, err),
    }

    let data: Value;

    match serde_json::from_str(&res) {
        Ok(json) => {
            data = json;
        }
        Err(err) => {
            println!(
                "Failed to parse {}.json, {}",
                name.to_string().bright_magenta(),
                err.to_string().bright_red(),
            );
            process::exit(1);
        }
    };

    data
}
