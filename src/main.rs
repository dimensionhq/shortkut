mod utils;

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::Instant;

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
                let shortcuts = &res["shortcuts"].as_array();
                let pb = ProgressBar::new(shortcuts.unwrap().len() as u64);

                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{bar:40.cyan/blue}")
                        .progress_chars("$$-"),
                );

                for shortcut in shortcuts.iter() {
                    let object = &shortcut[0];
                    let _alias: &str = &object["alias"].as_str().unwrap();
                    let _command: &str = &object["command"].as_str().unwrap();
                    thread::sleep(Duration::from_millis(100));
                    pb.inc(1);
                }

                pb.finish()
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
    let end = Instant::now();
    println!(
        "Added {} {} in {:.2}s",
        1.to_string().bright_green(),
        "shortcut",
        (end - start).as_secs_f32()
    );
}
