use colored::Colorize;
use minreq::get;
use std::env;

pub fn search(shell: String) {
    let args: Vec<String> = env::args().collect();
    let approx_string = &args[2];

    let data = match get("http://shortkut-api.herokuapp.com/api/v1/shortkut-list").send() {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "An error occured while requesting {}.json.\n\n{}: {}",
                "shortkut-list".bright_purple().bold(),
                "error".bright_red().bold(),
                err
            );
            std::process::exit(1);
        }
    };
    let shortkut_list = serde_json::from_str(data.as_str().unwrap()).unwrap();

    let matches = difflib::get_close_matches(approx_string, shortkut_list, 2, 0.6);

    if matches.len() == 1 {
        if matches[0] == approx_string {
            println!("{}", matches[0].bold().bright_green());
        } else {
            println!("{}", matches[0].bright_yellow());
        }
    } else if matches.len() == 0 {
        println!("{}", "No Matches Found!".bold().bright_red());
        std::process::exit(1);
    } else {
        for res in matches {
            if res.clone() == approx_string.as_str() {
                println!("{}", res.bold().bright_green());
            } else {
                println!("{}", res.bold().bright_yellow());
            }
        }
    }
}
