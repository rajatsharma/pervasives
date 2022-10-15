use std::env::current_dir;
use std::path::PathBuf;

pub fn relative(path: &str) -> Result<PathBuf, std::io::Error> {
    current_dir().map(|p| p.join(path))
}
