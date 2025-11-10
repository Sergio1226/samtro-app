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
    // let _ = services::user_repo::insert_user("admin2", "1234");
    // let _ = services::user_repo::find_user("admin2");

    // // Prueba para ver si funciona  y se agrega un producto
    // let product = models::product::Product {
    //     code: "TP01".into(),
    //     name: "Juan carlos bodoque".into(),
    //     category: "Marioneta".into(), 
    //     price: 125,
    //     stock: 12,
    //     minimum: 10,
    //     description: "Es una marioneta muy bonita de un color rojo vivo".into()
    // };

    // let _ = services::product_repo::insert_product(&product);
    // let copy = services::product_repo::find_product_by_code(product.code);

    // if let Ok(copy) = copy{
    //     if let Some(copy) = copy {
    //         println!("Hola me llamo {}", copy.name);
    //     }
    // }else{
    //     eprintln!("JAJAJAJAd");
    // }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::login
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}