use thiserror::Error;
use rusqlite;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Usuario o contraseña incorrectos")]
    InvalidCredentials,
    #[error("Error del sistema")]
    InternalError,
}
impl From<rusqlite::Error> for AppError {
    fn from(_err: rusqlite::Error) -> AppError {
        Self::InternalError
    }
}
pub type AppResult<T> = Result<T, AppError>;