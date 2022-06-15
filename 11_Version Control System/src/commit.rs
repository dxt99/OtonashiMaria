use std::fs;
use std::io::prelude::*;

pub fn commit() -> std::io::Result<()>{
    let mut file = fs::File::open(".geet/history.txt")?;
    let mut history = String::new();
    file.read_to_string(&mut history)?;
    Ok(())
}