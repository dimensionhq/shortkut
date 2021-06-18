use crate::helper::{generate_shortcut, generate_shortcut_multi};
use crate::model::shortkut::ShortKut;
use crate::utils;
use colored::Colorize;
use std::time::Instant;

pub fn add(shell: String) {
    println!("shc {} {}", "1.0.0", "add".bright_green().bold());

    let args: Vec<String> = std::env::args().collect();
    let start = Instant::now();

    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
    let mut installed: Vec<String> = vec![];

    for arg in vec.iter() {
        let res: ShortKut = utils::get_shortcut(arg);
        let shortcuts = res.shortcuts;

        for object in shortcuts.iter() {
            let alias: &str = object.alias.as_str();
            let is_array = object.command.is_array();

            if !is_array {
                let command = &object.command.as_str().unwrap();
                generate_shortcut(alias, command, shell.clone());
            } else {
                let commands = &object.command.as_array().unwrap();
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
}
