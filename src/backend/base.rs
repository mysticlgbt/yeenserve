// List of approved extensions.
pub static EXTENSIONS: &'static [&str] = &["jpg", "jpeg", "png"];

pub trait Backend: Send + Sync {
    fn list_files(&self) -> Result<Vec<String>, std::io::Error>;
    fn get_file_contents(&self, name: &str) -> Result<Vec<u8>, std::io::Error>;
}
