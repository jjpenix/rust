#![warn(clippy)]

fn main() {
    let s = Some(String::from("test string")).unwrap_or("Fail".to_string()).len();
}
