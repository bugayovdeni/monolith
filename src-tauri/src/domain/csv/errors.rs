use thiserror::Error;

/// Доменные ошибки CSV-парсера
#[derive(Debug, Error)]
pub enum CsvError {
    #[error("Файл не найден: {0}")]
    FileNotFound(String),

    #[error("Нет прав на чтение файла: {0}")]
    PermissionDenied(String),

    #[error("Неверный формат имени файла: {0}. Ожидается DataYYYYMMDD_HHMM.csv")]
    InvalidFilename(String),

    #[error("Ошибка парсинга даты из имени файла: {0}")]
    InvalidDate(String),

    #[error("Неверные заголовки. Ожидается 32 колонки, получено: {0}")]
    InvalidHeaders(usize),

    #[error("Ошибка парсинга строки {row}, поле '{field}': {reason}")]
    ParseError {
        row: usize,
        field: String,
        reason: String,
    },

    #[error("Файл пустой или не содержит данных")]
    EmptyFile,

    #[error("Ошибка кодировки файла: {0}")]
    EncodingError(String),

    #[error("Ошибка IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Ошибка CSV: {0}")]
    CsvError(#[from] csv::Error),
}

/// Результат работы парсера
pub type Result<T> = std::result::Result<T, CsvError>;
