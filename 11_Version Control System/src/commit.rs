use std::fs;
use std::io::prelude::*;

pub fn commit() -> std::io::Result<()>{
    let mut headlog = fs::File::open("./.geet/head.log")?;
    let mut head = String::new();
    let mut headlogtext = String::new();
    headlog.read_to_string(&mut headlogtext)?;

    //get head and parent
    let line = headlogtext.split(" ");
    let mut cnt = 0;
    for word in line{
        if cnt == 0 {head = word.to_string();}
        cnt+=1;
    }

    println!("{}", head);
    // reading temp
    Ok(())
}