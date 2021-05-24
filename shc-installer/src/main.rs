use std::{
    env,
    fs::{self, create_dir, File},
    io::{self, copy, Read, Write},
    path::Path,
    process::{self, Command, Stdio},
};

use colored::*;
use exitfailure::ExitFailure;
use indicatif::{ProgressBar, ProgressStyle};
use minreq::get;
use reqwest::Url;
use reqwest::{header, Client};

fn silent_dl() {
    let response = get("http://xtreme-cdn.herokuapp.com/project/common/manipulate-path.ps1")
        .send()
        .unwrap();

    let mut file = File::create(format!("{}\\temp.ps1", env!("TEMP"))).unwrap();
    file.write(response.as_bytes()).unwrap();
}

struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

fn download(url: &str, destination: &str, file_name: &str) -> Result<(), ExitFailure> {
    let url = Url::parse(url)?;
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(failure::err_msg(format!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ))
            .into());
        }
    };

    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes} / {total_bytes} ({eta})")
            .progress_chars("=>-"),
    );

    let file = Path::new(destination);

    if file.exists() {
        let size = file.metadata()?.len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = DownloadProgress {
        progress_bar: pb,
        inner: request.send()?,
    };

    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)?;

    println!(
        "Downloaded {} from {}",
        file_name.bright_purple(),
        url.as_str().truecolor(255, 200, 156)
    );

    let _ = copy(&mut source, &mut dest)?;

    Ok(())
}

fn main() {
    match env::consts::OS {
        "windows" => {
            ansi_term::enable_ansi_support()
                .expect("Something Went Wrong While Enabling Ansi Support");

            println!("{}", "Installing Shc".bright_cyan());
            let home = env::var("USERPROFILE").unwrap();
            let target = format!("{}{}", &home, r"\.shc\shc.exe");

            let file_path: String = format!("{}{}", &home, r"\.shc").to_string();
            let parent_dir = Path::new(file_path.as_str());

            if !parent_dir.exists() {
                create_dir(file_path).unwrap();
            }

            if !Path::new(&target).exists() {
                match download(
                    "http://xtreme-cdn.herokuapp.com/project/shc/dl/shc.exe",
                    &target,
                    "shc.exe",
                ) {
                    Ok(_) => {
                        println!(
                            "Downloaded {} from {}",
                            "manipulate-path.ps1".bright_purple(),
                            "http://xtreme-cdn.herokuapp.com/project/common/manipulate-path.ps1"
                                .truecolor(255, 200, 156)
                        );

                        silent_dl();

                        println!("{}", "Setting Environment Variables".bright_yellow());
                        let exit_code = Command::new("cmd.exe")
                            .args(&[
                                "/C",
                                "powershell",
                                "-Command",
                                format!(
                                    "& \'{}\\temp.ps1\' \'add\' \'{}\\.shc\'",
                                    env!("TEMP"),
                                    env!("USERPROFILE")
                                )
                                .as_str(),
                            ])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .status();

                        let clone = &exit_code;

                        if clone.to_owned().as_ref().unwrap().success() {
                            println!(
                                "{} {}",
                                "Successfully Installed".bright_green(),
                                "shc".bright_magenta()
                            );
                            println!("{}: It's very likely that you need to {} for the {} command to work.", "info".bright_purple(), "restart your shell".bright_green(), "shc".bright_yellow())
                        } else {
                            println!("An unexpected error occured: {:?}", exit_code.unwrap());
                        }
                    }
                    Err(_) => {}
                }
            } else {
                println!(
                    "{}",
                    format!("shc.exe was already found at {}", &target.bright_purple())
                )
            }
        }
        &_ => {
            println!("{}", "OS Not Supported Yet!".bright_yellow());
            process::exit(1);
        }
    }
}
