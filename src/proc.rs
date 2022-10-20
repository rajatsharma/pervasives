use std::process::{Command, Output, Stdio};

/// Calls command via fish eg. fish -c "echo hello"
pub fn call_command(command: &str) -> std::io::Result<Output> {
    Command::new("fish")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn call_command_(command: &str) -> Output {
    call_command(command).expect(&format!("could not run: {}", command))
}
