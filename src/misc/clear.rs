use std::process::Command;

pub fn main() {
    let output = Command::new("clear").output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
