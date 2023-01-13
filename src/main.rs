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
    // loop through the git logs and get last 3 commits
    let mut count = 0;
    for cap in re.captures_iter(&String::from_utf8_lossy(&output.stdout)) {
        if count == 3 {
            break;
        }
        println!("{}: {}", &cap[1], &cap[2]);
        count += 1;
    }

    /***********
     * Method 2
     ***********/

    // struct Commit {
    //     hash: String,
    //     message: String,
    // }

    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .filter_map(|line| pattern.captures(line))
    //     .map(|cap| Commit {
    //         hash: cap[1].to_string(),
    //         message: cap[2].trim().to_string(),
    //     })
    //     .take(5)
    //     .for_each(|x| println!("{:?}", x));
}
