#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tauri::{Manager, State};
use tokio::time::sleep;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            let app_handler = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(1000)).await;
                    app_handler.emit_all("keep-alive", "ping").unwrap();
                }
            });
            Ok(())
        })
        .manage(Counter::default())
        .invoke_handler(tauri::generate_handler![hello, count])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}

// invoke hello
#[tauri::command]
fn hello() -> String {
    "Hello World".into()
}

#[tauri::command]
fn count(n: i32, counter: State<'_, Counter>) -> i32 {
    let mut number = counter.0.lock().unwrap();
    *number += n;
    *number
}
