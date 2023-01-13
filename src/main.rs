use regex::Regex;
use std::process::Command;

fn main() {
    // get git logs
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .expect("failed to execute process");
    // regex pattern to match the commit hash and get message
    let re = Regex::new(r"([a-f0-9]{7}) (.*)").unwrap();
    // iterate over the git logs
    for cap in re.captures_iter(&String::from_utf8_lossy(&output.stdout)) {
        println!("{}: {}", &cap[1], &cap[2]);
    }
}
