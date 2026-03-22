use std::path::Path;

pub fn run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Путь в csv_manager {}", path.display());
    Ok(())
}
