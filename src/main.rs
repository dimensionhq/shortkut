// Modules
mod commands;
mod helper;
mod model;
mod utils;

// TODO: Allow Command File To Be A List Of Commands

fn main() {
    let shell = String::new();
    #[cfg(target_os = "linux")]
    let shell = std::env::var("SHELL").unwrap();
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    helper::initialize();
    utils::parse(shell);
}
