#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod models;
mod repo;
mod services;
mod errors;
mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    // // funcion que debería añadir un nuevo usuario en la db
    // services::user_repo::insert_user("admin", "1234");
    // services::user_repo::insert_user("admin2", "1234");
    // services::user_repo::insert_user("admin3", "1234");
    // services::user_repo::insert_user("admin4", "1234");

    // let user = services::user_repo::find_user("admin3");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::login
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}