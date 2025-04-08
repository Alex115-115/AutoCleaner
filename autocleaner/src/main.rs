use crate::{
    config::get_app_data_dir, config::get_config_path, notifier::scan_and_notify,
    settings::save_exec_path,
};
use gui::launch_gui;
use singleton_lock::acquire_singleton_lock;
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
mod notifier;
mod settings;
mod shared_flag;
mod singleton_lock;
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
///
/// ### Supported Modes:
///
/// - `--gui`  
///   Launches the graphical user interface (GUI).  
///   Prevents multiple instances by locking `gui.lock`.
///
/// - `--tray`  
///   Starts the system tray icon with menu options.  
///   Prevents multiple instances by locking `tray.lock`.
///
/// - `--tray-startup`  
///   Used at Windows startup. Performs a silent scan with notification,
///   then launches the tray icon.  
///   Also protected by `tray.lock`.
///
/// - *(no argument)*  
///   Saves the current executable path (for startup configuration),  
///   then launches both GUI and tray components.
///
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        return match args[1].trim_start_matches('-') {
            "gui" => {
                let lock_path = get_app_data_dir().join("gui.lock");
                let _singleton_guard = match acquire_singleton_lock(lock_path) {
                    Some(guard) => guard,
                    None => {
                        return Ok(());
                    }
                };

                launch_gui();

                Ok(())
            }
            "tray" => {
                let lock_path = get_app_data_dir().join("tray.lock");
                let _singleton_guard = match acquire_singleton_lock(lock_path) {
                    Some(guard) => guard,
                    None => {
                        return Ok(());
                    }
                };

                start_tray_icon();

                Ok(())
            }
            "tray-startup" => {
                let lock_path = get_app_data_dir().join("tray.lock");
                let _singleton_guard = match acquire_singleton_lock(lock_path) {
                    Some(guard) => guard,
                    None => {
                        return Ok(());
                    }
                };

                scan_and_notify();
                start_tray_icon();

                Ok(())
            }
            _ => Ok(()),
        };
    } else {
        if let Ok(path) = std::env::current_exe() {
            let _ = save_exec_path(path.to_string_lossy().to_string());
        }

        launch_gui_and_tray()?
    }

    Ok(())
}
