use colored::Colorize;
use serde_json::Value;
use std::env;
use std::fs::{create_dir, read_to_string, remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::iter::Iterator;
use std::path::Path;
use std::process;

#[allow(unused_variables)]
pub fn delete_shortcut_multi(alias: &str, command: &Vec<Value>, shell: String) {
    match env::consts::OS {
        "windows" => {
            let command_string: &String = &command
                .iter()
                .map(|value| format!("{}\n", value.to_string().replace("\"", "")))
                .collect::<String>();

            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");
            let file_path = format!("{}{}.bat", bin, alias);
            let contents = read_to_string(&file_path).unwrap_or_else(|_| {
                return String::new();
            });

            if contents.contains(&command_string.as_str()) {
                remove_file(&file_path).unwrap_or_else(|error| {
                    eprintln!("Failed To Delete Shortcut : {}", error);
                    process::exit(1);
                });
            }
        }
        &_ => {
            let location = String::new();

            #[cfg(target_os = "linux")]
            let location = get_shell_rc_location(shell);

            let data = read_to_string(&location).unwrap();

            let path = Path::new(location.as_str());
            if path.exists() {
                match OpenOptions::new().write(true).open(location) {
                    Ok(mut file) => {
                        let data_clone = data.clone();
                        let mut lines = data_clone.lines().collect::<Vec<&str>>().into_iter();

                        let lines_clone = lines.clone();

                        let mut start_index: i128 = -1;
                        let mut remove_lines_count = 0;

                        for (idx, line) in lines.by_ref().enumerate() {
                            if line.contains(alias) {
                                start_index = idx as i128;
                            } else if start_index != -1 {
                                if !line.contains("}") {
                                    remove_lines_count += 1;
                                } else {
                                    remove_lines_count += 1;
                                    break;
                                }
                            }
                        }

                        let lines_vec = lines_clone.collect::<Vec<&str>>();

                        if start_index != -1 {
                            file.set_len(0).unwrap();
                            let remove_lines = &lines_vec[start_index as usize
                                ..(start_index + remove_lines_count + 1) as usize];

                            let remove_string = &remove_lines.join("\n");
                            let write_string = data.replace(remove_string, "");

                            file.write(write_string.trim_end().as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn delete_shortcut(alias: &str, command: &str, shell: String) {
    match env::consts::OS {
        "windows" => {
            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");
            let file_path = format!("{}{}.bat", bin, alias);
            let contents = read_to_string(&file_path).unwrap_or_else(|_| {
                return String::new();
            });

            if contents.contains(&command) {
                remove_file(&file_path).unwrap_or_else(|error| {
                    eprintln!("Failed To Delete Shortcut : {}", error);
                    process::exit(1);
                });
            }
        }
        &_ => {
            let location = String::new();

            #[cfg(target_os = "linux")]
            let location = get_shell_rc_location(shell);

            let data = read_to_string(&location).unwrap();

            let path = Path::new(location.as_str());
            if path.exists() {
                match OpenOptions::new().write(true).open(location) {
                    Ok(mut file) => {
                        let data_clone = data.clone();
                        let mut lines = data_clone.lines().collect::<Vec<&str>>().into_iter();

                        let lines_clone = lines.clone();

                        let mut start_index: i128 = -1;
                        let mut remove_lines_count = 0;

                        for (idx, line) in lines.by_ref().enumerate() {
                            if line.contains(alias) {
                                start_index = idx as i128;
                            } else if start_index != -1 {
                                if !line.contains("}") {
                                    remove_lines_count += 1;
                                } else {
                                    remove_lines_count += 1;
                                    break;
                                }
                            }
                        }

                        let lines_vec = lines_clone.collect::<Vec<&str>>();

                        if start_index != -1 {
                            file.set_len(0).unwrap();
                            let remove_lines = &lines_vec[start_index as usize
                                ..(start_index + remove_lines_count + 1) as usize];

                            let remove_string = &remove_lines.join("\n");
                            let write_string = data.replace(remove_string, "");

                            file.write(write_string.trim_end().as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn generate_shortcut_multi(alias: &str, command: &Vec<Value>, shell: String) {
    match env::consts::OS {
        "windows" => {
            let command_string: &String = &command
                .iter()
                .map(|value| format!("{}\n", value.to_string().replace("\"", "")))
                .collect::<String>();

            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");

            let file = Path::new(&bin);

            if !file.exists() {
                match create_dir(&bin) {
                    Ok(_) => {
                        let location: String = format!("{}{}.bat", &bin, &alias);
                        let path = Path::new(location.as_str());
                        if !path.exists() {
                            let mut batch = File::create(location).expect("Failed To Create File");
                            batch
                                .write_all(format!("@echo off\n{}", command_string).as_bytes())
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "Failed To Create {} : {}",
                            ".shortkut".bright_red(),
                            err.to_string().bright_yellow()
                        );
                        process::exit(1);
                    }
                };
            } else {
                let bin: String =
                    format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");
                let location: String = format!("{}{}.bat", &bin, &alias);
                let path = Path::new(location.as_str());
                if !path.exists() {
                    let mut batch = File::create(location).expect("Failed To Create File");
                    batch
                        .write_all(format!("@echo off\n{}", command_string).as_bytes())
                        .unwrap();
                }
            }
        }
        &_ => {
            let commands = command
                .into_iter()
                .map(|v| {
                    v.to_string()
                        .replace("%", "$")
                        .replace("$*", "\"$@\"")
                        .replace("\"", "")
                })
                .collect::<Vec<String>>();

            let location = String::new();
            #[cfg(target_os = "linux")]
            let location = get_shell_rc_location(shell);

            let path = Path::new(location.as_str());

            if path.exists() {
                match OpenOptions::new().read(true).append(true).open(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let command = commands.join("\n    ");
                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                };
            } else {
                match File::create(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let command = commands.join("\n    ");
                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn generate_shortcut(alias: &str, command: &str, shell: String) {
    match env::consts::OS {
        "windows" => {
            let bin: String = format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");

            let file = Path::new(&bin);

            if !file.exists() {
                match create_dir(&bin) {
                    Ok(_) => {
                        let location: String = format!("{}{}.bat", &bin, &alias);
                        let path = Path::new(location.as_str());
                        if !path.exists() {
                            let mut batch = File::create(location).expect("Failed To Create File");
                            batch
                                .write_all(format!("@echo off\n{} %*", command).as_bytes())
                                .unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "Failed To Create {} : {}",
                            ".shortkut".bright_red(),
                            err.to_string().bright_yellow()
                        );
                        process::exit(1);
                    }
                };
            } else {
                let bin: String =
                    format!("{}\\{}", env::var("USERPROFILE").unwrap(), ".shortkut\\");
                let location: String = format!("{}{}.bat", &bin, &alias);
                let path = Path::new(location.as_str());
                if !path.exists() {
                    let mut batch = File::create(location).expect("Failed To Create File");
                    batch
                        .write_all(format!("@echo off\n{} %*", command).as_bytes())
                        .unwrap();
                }
            }
        }
        &_ => {
            let location = String::new();
            #[cfg(target_os = "linux")]
            let location = get_shell_rc_location(shell);

            let path = Path::new(location.as_str());

            if path.exists() {
                match OpenOptions::new().read(true).append(true).open(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                };
            } else {
                match File::create(location) {
                    Ok(mut file) => {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap_or_else(|err| {
                            println!("{}", err);
                            std::process::exit(1)
                        });

                        let write_string = format!(
                            r#"

function {}() {{
    {} "$@"
}}"#,
                            alias, command
                        );

                        // Write Alias To File (Append)
                        if !data.contains(&write_string) {
                            file.write_all(write_string.as_bytes()).unwrap();
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}",
                            format!(
                                "shortkut must be run with {} permissions",
                                "sudo".underline()
                            )
                            .bright_red()
                            .bold()
                        );
                        println!("{}", err);
                    }
                }
            }
        }
    }
}

pub fn initialize() {
    let directory = format!(r"{}/.shortkut", home::home_dir().unwrap().display());

    let path = Path::new(directory.as_str());

    if !path.exists() {
        create_dir(directory).unwrap();
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_shell_rc_location(shell: String) -> String {
    if shell.contains("bash") {
        return format!("{}/.bashrc", home::home_dir().unwrap().display());
    } else if shell.contains("zsh") {
        return format!("{}/.zshrc", home::home_dir().unwrap().display());
    } else {
        println!(
            "{} shell is not supported yet.",
            shell.bright_red().bold().underline()
        );
        std::process::exit(1);
    }
}
