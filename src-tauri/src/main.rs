#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn get_battery_percentage() -> String {
    "98% mockado_rust".into()
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![get_battery_percentage])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
