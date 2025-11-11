use crate::models::Product;
use crate::services::auth::AuthService;
use crate::errors::{AppError};
use crate::services::product_repo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserEntry {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct DefaultResponse{
    pub success: bool
}

// Cuando llamo login con el payload, llama  la funcion del auth
// si el auth dice q shi, retorno la respuesta que contiene un booleano string y el id del usuario
#[tauri::command]
pub async fn login(entry: UserEntry) -> Result<LoginResponse, String> {
    match AuthService::authenticate(&entry.username, &entry.password) {
        Ok(user) => Ok(LoginResponse {
            success: true,
            message: "Ingreso exitoso".into(),
            username: user.username,
        }),
        Err(AppError::InvalidCredentials) => Ok(LoginResponse {
            success: false,
            message: "Usuario o contraseña incorrectos".into(),
            username: "XDDDDDD".into(),
        }),
        Err(_) => Err("Error interno del servidor".into()),
    }
}

#[tauri::command]
pub async fn add_product(product: Product) -> Result<DefaultResponse, String> {
    match product_repo::insert_product(&product) {
        Ok(_product) => Ok(DefaultResponse { success: true }),
        Err(_) => Ok(DefaultResponse { success: false })
    }
}
