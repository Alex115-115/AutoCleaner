use crate::{
    config::get_config_path,
};
use gui::launch_gui;
use std::{
    env, fs,
    io::{self},
    os::windows::process::CommandExt,
};
use tray::start_tray_icon;
use winapi::um::winbase::CREATE_NO_WINDOW;

mod app;
mod cleanup;
mod config;
mod gui;
mod startup;
mod tray;

/// Launches both the GUI and the system tray icon as background processes.
fn launch_gui_and_tray() -> io::Result<()> {
    let _ = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("tray")
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    let _ = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("gui")
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    Ok(())
}

/// Entry point for the AutoCleaner application.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        return match args[1].trim_start_matches('-') {
            "gui" => {
                launch_gui();
                Ok(())
            }
            "tray" => {
                start_tray_icon();
                Ok(())
            }
            _ => Ok(()),
        };
    } else {
        launch_gui_and_tray()?
    }

    Ok(())
}
