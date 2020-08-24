use std::io::{self, Read};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b"a"[0], &mut buffer)?;
    println!("input was: {}", String::from_utf8(buffer).unwrap());
    Ok(())
}