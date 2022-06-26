use std::process::{Command, Stdio};
use std::str;

pub fn get_diff_files() -> String {
    let child = Command::new("git")
        .args("diff --cached --name-only --diff-filter=ACM".split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    assert!(output.status.success());
    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    strip_trailing_newline(s).to_string()
}

pub fn get_diff_file(file: &str) -> String {
    let command = format!("diff HEAD -U0 {}", file);
    let child = Command::new("git")
        .args(command.split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    assert!(output.status.success());
    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    strip_trailing_newline(s).to_string()
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

