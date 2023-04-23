use std::process::Command;

pub fn main()
{
    let dir = String::from("../testing");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("cargo new \"{}\" --bin", dir))
        .output()
        .unwrap();

    println!("{:?}", output);
}