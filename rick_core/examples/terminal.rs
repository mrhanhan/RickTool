use std::process::{Command, Stdio};

fn main() {
    let command =
        Command::new("powershell")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
    .output();
}