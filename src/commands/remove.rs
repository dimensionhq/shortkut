use crate::helper::{delete_shortcut, delete_shortcut_multi};
use crate::model::shortkut::ShortKut;
use crate::utils;
use colored::Colorize;
use std::time::Instant;

pub fn remove() {
    println!("shc {} {}", "1.0.0", "remove".bright_green().bold());

    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let vec: Vec<&str> = args[2].split(",").collect::<Vec<&str>>();
    let mut removed: Vec<String> = vec![];

    for arg in vec.iter() {
        let res: ShortKut = utils::get_shortcut(arg);
        let shortcuts = res.shortcuts;

        for object in shortcuts.iter() {
            let alias: &str = object.alias.as_str();
            let is_array = object.command.is_array();

            if !is_array {
                let command = &object.command.as_str().unwrap();
                delete_shortcut(alias, command);
            } else {
                let commands = &object.command.as_array().unwrap();
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
}
