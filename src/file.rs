use std::env::current_dir;
use std::path::PathBuf;

pub fn relative(path: &str) -> Result<PathBuf, std::io::Error> {
    current_dir().map(|p| p.join(path))
}

pub fn current_dir_name() -> String {
    let binding = current_dir().unwrap();
    binding
        .file_name()
        .clone()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
