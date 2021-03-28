use reqwest::blocking;

pub fn get_shortcuts() {
    blocking::get("https://google.com");
}
