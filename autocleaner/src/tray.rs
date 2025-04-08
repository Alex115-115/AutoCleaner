use crate::{
    startup::{is_startup_enabled, set_startup},
    CREATE_NO_WINDOW,
};
use std::os::windows::process::CommandExt;

use core::mem::MaybeUninit;
use trayicon::*;
use winapi::um::winuser;

/// Starts the system tray icon with context menu and background event handling.
pub fn start_tray_icon() {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Events {
        RightClickTrayIcon,
        LeftClickTrayIcon,
        DoubleClickTrayIcon,
        Exit,
        OpenGui,
        ToggleStartup,
    }
    use std::sync::{Arc, Mutex};
    let (s, r) = crossbeam_channel::unbounded();
    let icon = include_bytes!("../../resources/icon.ico");

    let icon = Icon::from_buffer(icon, None, None).unwrap();
    let run_at_startup_state = Arc::new(Mutex::new(is_startup_enabled()));
    let state_clone = run_at_startup_state.clone();

    fn build_menu(run_at_startup: bool) -> MenuBuilder<Events> {
        MenuBuilder::new()
            .checkable("Run at Startup", run_at_startup, Events::ToggleStartup)
            .separator()
            .item("Open GUI", Events::OpenGui)
            .separator()
            .item("Exit", Events::Exit)
    }

    let mut tray_icon = TrayIconBuilder::new()
        .sender(move |e| {
            let _ = s.send(*e);
        })
        .icon(icon.clone())
        .tooltip("AutoCleaner")
        .on_right_click(Events::RightClickTrayIcon)
        .on_click(Events::LeftClickTrayIcon)
        .on_double_click(Events::DoubleClickTrayIcon)
        .menu(build_menu(*run_at_startup_state.lock().unwrap()))
        .build()
        .unwrap();

    std::thread::spawn(move || {
        r.iter().for_each(|m| match m {
            Events::ToggleStartup => {
                let mut state = state_clone.lock().unwrap();
                *state = !*state;
                set_startup(*state);
                let new_menu = build_menu(*state);
                tray_icon.set_menu(&new_menu).unwrap();
            }
            Events::OpenGui => {
                let _ = std::process::Command::new(std::env::current_exe().unwrap())
                    .arg("gui")
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn();
            }
            Events::RightClickTrayIcon => {
                let run_at_startup_state = Arc::new(Mutex::new(is_startup_enabled()));
                let state_clone = run_at_startup_state.clone();
                let state = state_clone.lock().unwrap();
                let new_menu = build_menu(*state);
                tray_icon.set_menu(&new_menu).unwrap();

                tray_icon.show_menu().unwrap();
            }
            Events::DoubleClickTrayIcon => {}
            Events::LeftClickTrayIcon => {
                let run_at_startup_state = Arc::new(Mutex::new(is_startup_enabled()));
                let state_clone = run_at_startup_state.clone();
                let state = state_clone.lock().unwrap();
                let new_menu = build_menu(*state);
                tray_icon.set_menu(&new_menu).unwrap();

                tray_icon.show_menu().unwrap();
            }
            Events::Exit => {
                std::process::exit(0);
            }
        })
    });

    loop {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0);
            if bret > 0 {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                break;
            }
        }
    }
}
