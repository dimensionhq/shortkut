use reqwest::blocking;

pub fn get_shortcut(name: &str) {
    blocking::get("https://google.com");
}
