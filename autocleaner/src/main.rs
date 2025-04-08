use crate::{
    config::get_config_path,
};
use gui::launch_gui;
use std::{
    env, fs,
    io::{self},
};

mod cleanup;
mod config;
mod app;
mod gui;

/// Entry point for the AutoCleaner application.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        return match args[1].trim_start_matches('-') {
            "gui" => {
                launch_gui();
                Ok(())
            }
            _ => Ok(()),
        };
    } else {
        launch_gui();
    }

    Ok(())
}
