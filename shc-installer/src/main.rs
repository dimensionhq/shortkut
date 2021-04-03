use std::{
    env,
    fs::{self, create_dir},
    io::{self, copy, Read},
    path::Path,
    process::{self, Command},
};

use colored::*;
use exitfailure::ExitFailure;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use reqwest::{header, Client};

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
    let pb = ProgressBar::new(total_size - 150000);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes} / {total_bytes} ({eta})")
            .progress_chars("#>-"),
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
        "Downloading {} from {}",
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

            let home = env::var("USERPROFILE").unwrap();
            let target = format!("{}{}", &home, r"\.shc\shc.exe");

            let file_path: String = format!("{}{}", &home, r"\.shc").to_string();
            let parent_dir = Path::new(file_path.as_str());

            if !parent_dir.exists() {
                create_dir(file_path).unwrap();
            }

            if !Path::new(&target).exists() {
                match download(
                    "https://xtreme-cdn.herokuapp.com/project/shc/dl/shc.exe",
                    &target,
                    "shc.exe",
                ) {
                    Ok(_) => {
                        println!("{}", "Installing Shc".bright_cyan());

                        match download(
                            "https://xtreme-cdn.herokuapp.com/project/common/manipulate-path.ps1",
                            format!(r"{}\temp.ps1", env::var("TEMP").unwrap()).as_str(),
                            "manipulate-path.ps1",
                        ) {
                            Ok(_) => {
                                println!("{}", "Setting Environment Variables".bright_yellow());
                                Command::new("powershell.exe")
                                    .arg("-NoProfile")
                                    .arg("-NonInteractive")
                                    .arg("-File")
                                    .arg(format!(r"{}\temp.ps1", env::var("TEMP").unwrap()))
                                    .spawn()
                                    .unwrap();

                                println!(
                                    "{} {}",
                                    "Successfully Installed".bright_green(),
                                    "shc".bright_magenta()
                                );
                            }
                            Err(_) => {}
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
            println!("{}", "OS Not Supported!".bright_yellow());
            process::exit(1);
        }
    }
}
