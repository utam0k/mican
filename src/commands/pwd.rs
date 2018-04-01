use std::env;

pub fn run() -> Result<(), String> {
    println!("{:?}", env::current_dir().unwrap());
    return Ok(());
}
