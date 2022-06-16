// add checks 
use std::fs::OpenOptions;
use std::io::prelude::*;

// maps every file in the directory to a number
pub fn add() -> std::io::Result<()>{

    Ok(())
}

pub fn remove()-> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./.geet/temp.txt")
        .unwrap();
    file.write_all(b"")?;
    Ok(())
}