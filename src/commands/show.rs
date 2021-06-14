use colored::Colorize;
use std::env;
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io::Read;
use std::process;
use std::time::Instant;

use crate::model::shortkut::ShortKut;
use crate::utils;

pub fn show() {
    println!("shc {} {}", "1.0.0", "show".bright_green().bold());

    let args: Vec<String> = std::env::args().collect();
    let start = Instant::now();

    let paths = read_dir(format!("{}{}", env::var("USERPROFILE").unwrap(), "\\.shc")).unwrap();

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

            println!(
                "{} {} {}",
                &args[2].bright_cyan(),
                "=".bright_magenta(),
                command
            );
            process::exit(0);
        }
    }

    let res: ShortKut = utils::get_shortcut(&args[2]);
    let shortcuts = &res.shortcuts;

    for object in shortcuts.iter() {
        // let alias: &str = &object["alias"].as_str().unwrap();
        let is_array = object.command.is_array();
        let alias = &object.alias.as_str();

        if !is_array {
            let command = &object.command.as_str().unwrap();

            println!(
                "{} {} {}",
                alias.bright_cyan(),
                "=".bright_magenta(),
                command
            );
        } else {
            let description = &object.clone().description.unwrap();

            println!("{}", termimad::inline(description.as_str()));
        }
    }

    let end = Instant::now();

    let shortcut: &str;
    if shortcuts.len() == 1 {
        shortcut = "shortcut"
    } else {
        shortcut = "shortcuts"
    }

    println!(
        "Found {} {} in {:.2}s",
        shortcuts.len().to_string().bright_green(),
        shortcut,
        (end - start).as_secs_f32()
    );
}
