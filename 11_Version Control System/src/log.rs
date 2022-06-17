use std::fs;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

pub fn log() -> std::io::Result<()>{
    let mut history = fs::File::open("./.geet/history.log")?;
    let mut historytext = String::new();
    history.read_to_string(&mut historytext)?;

    if historytext.len() == 0 {
        println!("No commits made");
    } else {
        let lines = historytext.split("\n");
        for line in lines{
            let vec :Vec<&str> = line.split(" ").collect();
            if vec.len()<=1 {continue;}
            if vec.len()!=4 {return Err(Error::new(ErrorKind::InvalidData, "Corrupt log"))} //assert
            // {0,1,2,3,4} -> {Head, Parent, Msg, Date, Time}
            
            println!("Commit ID {}", &vec[0]);
            if vec[1] == "0" {println!("Parent: None");}
            else {println!("Parent: {}", &vec[1])};
            let date = vec[3].replace("%20%", " ");
            println!("Date: {}", &date);
            let msg = vec[2].replace("%20%", " ");
            println!("Message: {}", &msg);
            println!("");
        }
    }

    Ok(())
}