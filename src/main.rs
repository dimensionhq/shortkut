mod utils;

use colored::*;
use std::env;
use utils::*;


const __version__: &str = "1.0.0";
fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &str = &args[1];
    // https://rust-lang-nursery.github.io/rust-cookbook/cli/ansi_terminal.html
    match command {
        "add" => {
            println!("shc {} {}", __version__, "add".green().bold());
            
        },
        "remove" =>{
            println!("shc {} {}", __version__, "remove".green().bold());
        },
        _ => println!("Invalid Command!"),
    }
    // get()

    // write, i don't write
    // then we need to create shortcuts

    // then we need to print success
}
