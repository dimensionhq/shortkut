// Modules
mod commands;
mod helper;
mod model;
mod utils;

// TODO: Allow Command File To Be A List Of Commands

fn main() {
    #[allow(unused_variables)]
    let shell = String::new();
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let shell = std::env::var("SHELL").unwrap();
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    helper::initialize();
    utils::parse(shell);
}
