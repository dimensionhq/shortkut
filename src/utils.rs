use colored::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use reqwest::blocking;
use scraper::{Html, Selector};
use serde_json::Value;
use std::process;

pub fn get_shortcut(name: &str) -> Value {
    let mut res: String = String::new();

    match blocking::get(format!(
        "https://raw.githubusercontent.com/XtremeDevX/shc/master/shortcuts/{}.json",
        name
    )) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                // Response Code Is 200 OK
                match response.text() {
                    Ok(text) => {
                        res = text;
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            "Failed To Parse Response:".to_string().yellow(),
                            e.to_string().bright_red()
                        );
                        process::exit(1);
                    }
                }
            } else {
                println!(
                    "{} is not a valid shortcut pack.",
                    name.to_string().bright_magenta()
                );
                process::exit(1);
            }
        }
        Err(err) => println!("Failed To Request {}.json => {}", name, err),
    }

    let data: Value;

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

pub fn send_search_query(approx: String) -> String {
    let mut html: String = String::new();

    match blocking::get("https://github.com/XtremeDevX/shc/tree/master/shortcuts") {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                // Response Code Is 200 OK
                match response.text() {
                    Ok(text) => {
                        html = text;
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            "Failed To Parse Response:".to_string().yellow(),
                            e.to_string().bright_red()
                        );
                        process::exit(1);
                    }
                }
            } else {
                println!("Something went wrong! Return Code is not 200 OK!",);
                process::exit(1);
            }
        }
        Err(err) => println!("Failed To Request Package List From Github => {}", err),
    }

    let parsed_html = Html::parse_document(html.as_str());

    let selector = Selector::parse("a").unwrap();
    let elements = parsed_html.select(&selector);
    let mut options: Vec<String> = vec![];

    for element in elements {
        let text = element.text().collect::<Vec<_>>()[0].to_string();
        if text.ends_with(".json") {
            options.push(text.replace(".json", ""));
        }
    }

    let searcher = SkimMatcherV2::default();
    let mut best_match: String = String::new();
    let mut max = 0;
    let mut idx = 0;

    for element in options.iter() {
        if let Some(result) = searcher.fuzzy_match(&approx, element) {
            if result > max {
                max = result;
                best_match = options[idx].to_string();
            }
        }

        idx += 1
    }

    best_match = best_match.replace("\"", "");

    best_match.to_string()
}

