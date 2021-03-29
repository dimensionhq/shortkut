mod utils;

use colored::*;
use std::env;
use std::time::Instant;

const __VERSION__: &str = "1.0.0";

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let command: &str = &args[1];

    match command {
        "add" => {
            println!("shc {} {}", __VERSION__, "add".green().bold());

            // 2 Possibilities : shc add cargo and shc add cru < cargo run
            // TODO: Handle Multiple Shortcuts shc add cargo,git

            if args.len() == 3 {
                let response = utils::get_shortcut(&args[2]);
                println!(
                    "Adding {}",
                    &response["display-name"].to_string().bright_cyan()
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
    let end = Instant::now();
    println!(
        "Added {} {} in {:.2}s",
        1.to_string().bright_green(),
        "shortcut",
        (end - start).as_secs_f32()
    );
}
