use std;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, Error, ErrorKind};

pub fn head(head: &String) -> std::io::Result<()> {
    // read commit
    let mut commitname = "./.geet/commits/".to_string();
    commitname.push_str(&head);
    commitname.push_str(".cmt");
    let mut commit = fs::File::open(&commitname)?;
    let mut committext = String::new();
    commit.read_to_string(&mut committext)?;

    // move to map
    let mut files: HashMap<String, String> = HashMap::new();
    let lines = committext.split("\n");
    for line in lines{
        let vec :Vec<&str> = line.split(" ").collect();
        if vec.len()<2 {continue;}
        files.insert(vec[0].to_string(), vec[1].to_string());
    }

    // implement commit
    for (path, snap) in files{
        let mut pathsnap = "./.geet/snapshots/".to_string();
        pathsnap.push_str(&snap);
        pathsnap.push_str(".snp");

        let mut target = std::fs::File::open(&path)?;
        let mut source = std::fs::File::create(&pathsnap)?;
        std::io::copy(&mut source, &mut target)?;
    }

    // find head and parent
    let mut parent = String::new();
    let mut history = fs::File::open("./.geet/history.log")?;
    let mut historytext = String::new();
    history.read_to_string(&mut historytext)?;
    let lines = historytext.split("\n");
    for line in lines{
        let vec :Vec<&str> = line.split(" ").collect();
        if vec.len()<=1 {continue;}
        if vec.len()!=4 {return Err(Error::new(ErrorKind::InvalidData, "Corrupt log"))} //assert
        if vec[0] == head {
            parent = vec[1].to_string().clone();
            break;
        }
    }

    // update head.log
    let mut buf = head.to_string();
    buf.push_str(" ");
    buf.push_str(&parent);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./.geet/head.log")
        .unwrap();
    file.write_all(buf.as_bytes())?;

    
    Ok(())
}