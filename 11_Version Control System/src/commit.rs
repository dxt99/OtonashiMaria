use std::fs;
use std::io::prelude::*;

pub fn commit() -> std::io::Result<()>{
    let mut file = fs::File::open(".geet/history.log")?;
    let mut history = String::new();
    file.read_to_string(&mut history)?;
    if history.len() == 0{
        println!("First commit");
    } else {

    }
    Ok(())
}