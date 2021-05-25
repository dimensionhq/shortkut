// Modules
mod commands;
mod helper;
mod model;
mod utils;

// TODO: Allow Command File To Be A List Of Commands

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    helper::initialize();
    utils::parse();
}
