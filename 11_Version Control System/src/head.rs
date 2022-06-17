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

    // delete all other tracked files
    let mut headlog = fs::File::open("./.geet/head.log")?;
    let mut oldhead = String::new();
    let mut headlogtext = String::new();
    headlog.read_to_string(&mut headlogtext)?;

    //  get head
    let line = headlogtext.split(" ");
    let mut cnt = 0;
    for word in line{
        if cnt == 0 {oldhead = word.to_string();}
        cnt+=1;
    }
    //  read commit
    let mut commitname = "./.geet/commits/".to_string();
    commitname.push_str(&oldhead);
    commitname.push_str(".cmt");
    let mut commit = fs::File::open(&commitname)?;
    let mut committext = String::new();
    commit.read_to_string(&mut committext)?;

    //  delete from old commit
    let lines = committext.split("\n");
    for line in lines{
        let vec :Vec<&str> = line.split(" ").collect();
        if vec.len()<2 {continue;}
        if std::path::Path::new(&vec[0]).exists() {fs::remove_file(&vec[0])?;}
    }

    // implement commit
    for (path, snap) in files{
        let mut pathsnap = "./.geet/snapshots/".to_string();
        pathsnap.push_str(&snap);
        pathsnap.push_str(".snp");

        let pathtar = std::path::Path::new(&path);
        let prefix = pathtar.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut target = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        let mut source = std::fs::File::open(&pathsnap)?;
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

    println!("Head changed");
    Ok(())
}