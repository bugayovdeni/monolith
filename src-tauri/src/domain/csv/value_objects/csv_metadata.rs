use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Метаданные CSV файла
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CsvMetadata {
    pub file_path: String,
    pub file_name: String,
    pub total_rows: usize,
    pub total_columns: usize,
    pub columns_used: usize,
    pub file_size_bytes: u64,
    pub memory_size_bytes: usize,
    pub parsed_at: DateTime<Utc>,
}

impl CsvMetadata {
    pub fn new(
        file_path: String,
        file_name: String,
        total_rows: usize,
        file_size_bytes: u64,
        record_size: usize,
    ) -> Self {
        let memory_size = total_rows * record_size;

        Self {
            file_path,
            file_name,
            total_rows,
            total_columns: 32,
            columns_used: 16,
            file_size_bytes,
            memory_size_bytes: memory_size,
            parsed_at: Utc::now(),
        }
    }

    pub fn file_size_human(&self) -> String {
        Self::format_bytes(self.file_size_bytes)
    }

    pub fn memory_size_human(&self) -> String {
        Self::format_bytes(self.memory_size_bytes as u64)
    }

    fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        match bytes {
            b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
            b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
            b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
            b => format!("{} B", b),
        }
    }

    pub fn compression_ratio(&self) -> Option<f64> {
        if self.file_size_bytes == 0 {
            return None;
        }
        let ratio = (self.memory_size_bytes as f64) / (self.file_size_bytes as f64);
        Some(ratio * 100.0)
    }

    pub fn is_within_limits(&self, max_memory_mb: usize) -> bool {
        let limit_bytes = max_memory_mb * 1024 * 1024;
        self.memory_size_bytes <= limit_bytes
    }
}

// ==================== ТЕСТЫ ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metadata() -> CsvMetadata {
        CsvMetadata::new(
            "/path/to/Data20260123_1339.csv".to_string(),
            "Data20260123_1339.csv".to_string(),
            10_000,
            2_500_000,
            52,
        )
    }

    #[test]
    fn test_new_metadata() {
        let meta = sample_metadata();
        assert_eq!(meta.file_name, "Data20260123_1339.csv");
        assert_eq!(meta.total_rows, 10_000);
        assert_eq!(meta.total_columns, 32);
        assert_eq!(meta.columns_used, 16);
        assert_eq!(meta.memory_size_bytes, 10_000 * 52);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(CsvMetadata::format_bytes(512), "512 B");
        assert_eq!(CsvMetadata::format_bytes(2048), "2.00 KB");
        assert_eq!(CsvMetadata::format_bytes(2_500_000), "2.38 MB");
        assert_eq!(CsvMetadata::format_bytes(1_500_000_000), "1.40 GB");
    }

    #[test]
    fn test_size_human() {
        let meta = sample_metadata();
        assert!(meta.file_size_human().contains("MB"));
        assert!(meta.memory_size_human().contains("KB"));
    }

    #[test]
    fn test_compression_ratio() {
        let meta = sample_metadata();
        let ratio = meta.compression_ratio().unwrap();
        assert!((20.0..22.0).contains(&ratio));
    }

    #[test]
    fn test_within_limits() {
        let meta = sample_metadata();
        assert!(meta.is_within_limits(10));
        assert!(!meta.is_within_limits(0));
    }

    #[test]
    fn test_empty_file_ratio() {
        let meta = CsvMetadata::new("/empty.csv".to_string(), "empty.csv".to_string(), 0, 0, 52);
        assert_eq!(meta.compression_ratio(), None);
    }
}
