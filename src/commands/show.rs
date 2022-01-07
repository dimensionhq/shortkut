use colored::Colorize;
use std::env;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::fs::{read_dir, File};
use std::io::Read;
use std::process;

#[cfg(target_os = "linux")]
use crate::helper::get_shell_rc_location;
use crate::model::shortkut::ShortKut;
use crate::utils;

#[allow(unused_variables)]
pub fn show(shell: String) {
    println!("shortkut {} {}", "1.0.0", "show".bright_green().bold());

    let args: Vec<String> = std::env::args().collect();

    match std::env::consts::OS {
        "windows" => {
            let paths = read_dir(format!(
                "{}{}",
                env::var("USERPROFILE").unwrap(),
                "\\.shortkut"
            ))
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
                        "\\.shortkut\\",
                        &file_name
                    ))
                    .unwrap();
                    file.read_to_string(&mut command).unwrap();
                    command = command
                        .replace("@echo off", "")
                        .replace("%*", "")
                        .replace("\n", "\n  ")
                        .trim_end()
                        .to_string();

                    let lines = command.lines().collect::<Vec<&str>>();
                    if lines.len() == 1 {
                        println!(
                            "{} {} {}",
                            &args[2].bright_cyan(),
                            "=".bright_magenta(),
                            command
                        );
                    } else {
                        println!(
                            "{} {{ {}\n}}",
                            &args[2].bright_cyan().bold(),
                            command.yellow()
                        );
                    }

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
                        "=".bright_magenta().bold(),
                        command
                    );
                } else {
                    let description = &object.clone().description.unwrap();

                    println!("{}", termimad::inline(description.as_str()));
                }
            }
        }
        &_ => {
            let location = String::new();

            #[cfg(target_os = "linux")]
            let location = get_shell_rc_location(shell);
            let data = read_to_string(location).unwrap();

            let pack = &args[2];
            let mut start_index: i128 = -1;

            for (idx, line) in data.lines().enumerate() {
                if line.contains(pack) && line.starts_with("function") {
                    start_index = idx as i128;
                }
            }
            if start_index == -1 {
                let res: ShortKut = utils::get_shortcut(&pack);
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
                            "=".bright_magenta().bold(),
                            command
                        );
                    } else {
                        let description = &object.clone().description.unwrap();

                        println!("{}", termimad::inline(description.as_str()));
                    }
                }
            } else {
                let lines = data.lines().collect::<Vec<&str>>();

                let revised = &lines[start_index as usize + 1..];

                let mut final_vec: Vec<String> = vec![];

                for v in revised {
                    if v.trim() != "}" {
                        final_vec.push(v.trim().replace("\"$@\"", ""));
                    } else {
                        break;
                    }
                }

                let command = final_vec.join("\n  ");

                if final_vec.len() > 1 {
                    // Multiline Command
                    println!(
                        "{} {{ \n  {} \n}}",
                        pack.bright_cyan().bold(),
                        command.yellow()
                    );
                } else {
                    println!(
                        "{} {} {}",
                        pack.bright_cyan().bold(),
                        "=".bright_magenta(),
                        command
                    );
                }
            }
        }
    }
}
