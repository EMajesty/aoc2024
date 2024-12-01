use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input1_1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(())
}
