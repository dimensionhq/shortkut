use colored::Colorize;
use minreq::get;
use std::env;

pub fn search() {
    let args: Vec<String> = env::args().collect();
    let approx_string = &args[2];

    let data = match get("http://shortkut-api.us-east-1.elasticbeanstalk.com/api/v1/shortkut-list")
        .send()
    {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "\nAn error occured while requesting {}.json.\n{}: {}",
                "shortkut-list".bright_purple().bold(),
                "error".bright_red().bold(),
                err
            );
            std::process::exit(1);
        }
    };
    let shortkut_list = serde_json::from_str(data.as_str().unwrap()).unwrap();

    let matches = difflib::get_close_matches(approx_string, shortkut_list, 1, 0.6);

    let mut response = "";

    if matches.len() == 1 {
        response = matches[0];
    }

    if response != "" {
        if response.clone() == approx_string.as_str() {
            println!("{}", response.bold().bright_green());
        } else {
            println!("{}", response.bold().bright_yellow());
        }
    } else {
        println!("{}", "No Matches Found!".bold().bright_red());
        std::process::exit(1);
    }
}
