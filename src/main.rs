#![allow(unused)]

use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("gh repo view nogtk/nogtk.dev")
        .output()
        .expect("error");
    let result = output.stdout;
    println!("{}", std::str::from_utf8(&result).unwrap());
}
