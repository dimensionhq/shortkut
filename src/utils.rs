use crate::commands::{add, show};
use crate::commands::{remove, search};
use crate::helper::{delete_shortcut, generate_shortcut};
use crate::model::shortkut::ShortKut;
use colored::*;
use minreq::get;
use std::{process, time::Instant};

pub fn get_shortcut(name: &str) -> ShortKut {
    let mut res: String = String::new();

    match get(format!("http://shortkut-api.herokuapp.com/api/v1/{}", name)).send() {
        Ok(data) => {
            res = data.as_str().unwrap().to_string();
        }
        Err(err) => {
            eprintln!(
                "\nAn error occured while requesting {}.json.\n{}: {}",
                name.bright_purple().bold(),
                "error".bright_red().bold(),
                err
            );
        }
    }

    let data: ShortKut;
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

pub fn parse(shell: String) {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            // Display Help Menu
            let help = format!(
                r#"{}
{} add - Add a shortcut
{} remove - Remove a shortcut
{} show - Show a shortcut pack
{} search - Search for a shortcut pack"#,
                format!("shc {}", "1.0.0".bright_green()),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
            );
            println!("{}", help);
        }
        2 => {
            if args[1] == "--version" {
                println!("shc version {}", "1.0.0".bright_green().bold())
            } else {
                match args[1].as_str() {
                    "add" => {
                        let error = format!(
                            r#"{}: expected a shortkut pack or alias-command pair to add.

usage:
  {} add {} or {} add {} {}

examples:
  {} add flutter  {}
  {} add npm,yarn  {}
  {} add cru {}  {}

links: 
  {}: {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "<alias>".bright_cyan(),
                            "<command>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "// add flutter shortkut pack".bright_black(),
                            "shc".bright_green().bold(),
                            "// add npm and yarn shortkut packs".bright_black(),
                            "shc".bright_green().bold(),
                            "\"cargo run\"".bright_yellow(),
                            "// command \"cargo run\" aliased to cru".bright_black(),
                            "1".blue(),
                            "https://shortkut.sh/docs/add".bright_purple().underline(),
                        );

                        println!("{}", error);
                        process::exit(1);
                    }
                    "remove" => {
                        let error = format!(
                            r#"{}: expected a shortkut pack or alias-command pair to remove.

usage:
  {} remove {} or {} remove {} {}

examples:
  {} remove flutter  {}
  {} remove npm,yarn  {}
  {} remove cru {}  {}

links: 
  {}: {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "<alias>".bright_cyan(),
                            "<command>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "// remove flutter shortkut pack".bright_black(),
                            "shc".bright_green().bold(),
                            "// remove npm and yarn shortkut packs".bright_black(),
                            "shc".bright_green().bold(),
                            "\"cargo run\"".bright_yellow(),
                            "// remove command \"cargo run\" aliased to cru".bright_black(),
                            "1".blue(),
                            "https://shortkut.sh/docs/remove"
                                .bright_purple()
                                .underline(),
                        );

                        println!("{}", error);
                        process::exit(1);
                    }
                    "show" => {
                        let error = format!(
                            r#"{}: expected name of a shortkut pack to display.
usage:
  {} show {}

examples:
  {} show {}
  
links: 
  {}: {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan().bold(),
                            "shc".bright_green().bold(),
                            "flutter".bright_cyan().bold(),
                            "1".blue(),
                            "https://shortkut.sh/docs/show".bright_purple().underline(),
                        );

                        println!("{}", error);
                        process::exit(1);
                    }
                    "search" => {
                        let error = format!(
                            r#"{}: expected approximate name of a shortkut pack to search for.
usage:
  {} search {}

examples:
  {} search {}
  {} search {}

links:
  {}: {}
  "#,
                            "error".bright_red().bold(),
                            "shc".bold().bright_green(),
                            "<approx-name>".bold().bright_cyan(),
                            "shc".bold().bright_green(),
                            "flutter".bold().bright_cyan(),
                            "shc".bold().bright_green(),
                            "yarn".bold().bright_cyan(),
                            "1".bright_blue(),
                            "https://shortkut.sh/docs/search"
                                .bright_magenta()
                                .underline()
                        );

                        println!("{}", error);
                        process::exit(1);
                    }
                    _ => {
                        println!(
                            "{}: {} is not a valid command.",
                            "error".bright_red().bold(),
                            args[1].bright_cyan()
                        );
                        process::exit(1);
                    }
                }
            }
        }
        3 => match args[1].as_str() {
            "add" => {
                add::add(shell.clone());
                if vec!["linux", "macos"].contains(&std::env::consts::OS.to_string().as_str()) {
                    match shell.as_str() {
                        "/bin/bash" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.bashrc".bright_cyan().bold()
                            );
                        }
                        "usr/bin/zsh" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.zshrc".bright_cyan().bold()
                            );
                        }
                        _ => {}
                    }
                };
            }
            "remove" => {
                remove::remove(shell.clone());

                if vec!["linux", "macos"].contains(&std::env::consts::OS.to_string().as_str()) {
                    match shell.as_str() {
                        "/bin/bash" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.bashrc".bright_cyan().bold()
                            );
                        }
                        "/usr/bin/zsh" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.zshrc".bright_cyan().bold()
                            );
                        }
                        _ => {}
                    }
                };
            }
            "show" => {
                show::show(shell);
            }
            "search" => {
                search::search();
                let end = Instant::now();
                println!(
                    "Found {} {} in {:.2}s",
                    "1".to_string().bright_green(),
                    "shortcut",
                    (end - start).as_secs_f32()
                );
            }
            _ => {
                println!(
                    "{}: {} is not a valid command.",
                    "error".bright_red().bold(),
                    args[1].bright_cyan()
                );
                process::exit(1);
            }
        },
        4 => match args[1].as_str() {
            "add" => {
                println!("shc {} {}", "1.0.0", "add".bright_green().bold());

                let alias = &args[2];
                let command = &args[3].to_string().replace("\"", "");
                generate_shortcut(alias, command, shell.clone());
                let end = Instant::now();
                println!(
                    "Added {} shortcut in {:.2}s",
                    1.to_string().bright_green(),
                    (end - start).as_secs_f32()
                );

                if vec!["linux", "macos"].contains(&std::env::consts::OS.to_string().as_str()) {
                    match shell.as_str() {
                        "/bin/bash" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.bashrc".bright_cyan().bold()
                            );
                        }
                        "/usr/bin/zsh" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.zshrc".bright_cyan().bold()
                            );
                        }
                        _ => {}
                    }
                };
            }
            "remove" => {
                println!("shc {} {}", "1.0.0", "remove".bright_green().bold());

                let alias = &args[2];
                let command = &args[3].to_string().replace("\"", "");
                delete_shortcut(alias, command, shell.clone());
                let end = Instant::now();
                println!(
                    "Removed {} shortcut in {:.2}s",
                    1.to_string().bright_green(),
                    (end - start).as_secs_f32()
                );

                if vec!["linux", "macos"].contains(&std::env::consts::OS.to_string().as_str()) {
                    match shell.as_str() {
                        "/bin/bash" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.bashrc".bright_cyan().bold()
                            );
                        }
                        "/usr/bin/zsh" => {
                            println!(
                                "Use {} to start using added shortkuts.",
                                "source ~/.zshrc".bright_cyan().bold()
                            );
                        }
                        _ => {}
                    }
                };
            }
            _ => {}
        },
        _ => {
            println!(
                "{}: shc recieved unexpected arguments",
                "error".bright_red().bold()
            );
        }
    }
}
