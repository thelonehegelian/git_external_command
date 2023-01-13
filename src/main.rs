use regex::Regex;
use std::process::Command;

fn main() {
    // get git logs
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .expect("failed to execute process");
}
