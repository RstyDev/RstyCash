use chrono::ParseError;
use core::num::ParseIntError;
use std::{env::VarError, io, num::ParseFloatError, time::SystemTimeError};

use thiserror::Error;
pub type Res<T> = std::result::Result<T, AppError>;
pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de monto, el monto a pagar es: {a_pagar:?},el monto pagado es: {pagado:?}")]
    AmountError { a_pagar: f32, pagado: f32 },
    #[error("Error de .env")]
    EnvError(#[from] VarError),
    #[error("Error de {0}")]
    IncorrectError(String),
    #[error("Solo existen dos posiciones para venta")]
    SaleSelection,
    #[error("Presentacion seleccionada incorrecta, no existe {0}")]
    SizeSelection(String),
    #[error("{objeto:?} {instancia:?} existente")]
    ExistingError { objeto: String, instancia: String },
    #[error("No encontrado el {objeto:?} de id {instancia:?}")]
    NotFound { objeto: String, instancia: String },
    #[error("Error de archivo")]
    FileSystemError(#[from] io::Error),
    #[error("Error de hora del sistema")]
    SystemTimeError(#[from] SystemTimeError),
    #[error("Error de conversion de flotante")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Error de conversion de enteros")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error de conversion")]
    ParseError,
    #[error("Error de conversion de fecha")]
    ChronoParseError(#[from] ParseError),
    #[error("Error de inicialización {0}")]
    InicializationError(String),
    #[error("Error de bases de datos")]
    DbError(#[from] sqlx::Error),
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.to_string()
    }
}
