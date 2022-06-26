use std::process::{Command, Stdio};
use std::str;

fn main() {
    let child = Command::new("git")
        .args("diff --cached --name-only --diff-filter=ACM".split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    assert!(output.status.success());
    //println!("{:?}", output.stdout);
    let s = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}", s);
}
