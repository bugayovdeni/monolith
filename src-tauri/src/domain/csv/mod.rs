// domain/csv/mod.rs

pub mod entities;
pub mod errors;
pub mod services;
pub mod value_objects; // ← pub обязательно!

// Ре-экспорты для удобства
pub use entities::{CementingData, CementingRecord};
pub use errors::{CsvError, Result};
pub use value_objects::CementingUnits; // ← или отсюда
