use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::collections::HashMap;
use chrono;

pub fn commit(msg: &String) -> std::io::Result<()>{
    let mut headlog = fs::File::open("./.geet/head.log")?;
    let mut head = String::new();
    let mut headlogtext = String::new();
    headlog.read_to_string(&mut headlogtext)?;

    // get head and parent
    let line = headlogtext.split(" ");
    let mut cnt = 0;
    for word in line{
        if cnt == 0 {head = word.to_string();}
        cnt+=1;
    }

    // process temp
    let mut templog = fs::File::open("./.geet/temp.log")?;
    let mut templogtext = String::new();
    templog.read_to_string(&mut templogtext)?;

    if templogtext.len()==0 {
        println!("No changes staged (run 'geet add' before commiting)");
        return Ok(());
    }

    let mut files: HashMap<String, String> = HashMap::new();
    let lines = templogtext.split("\n");
    for line in lines{
        let mut cnt = 0;
        let words = line.split(" ");
        let mut key = String::new();
        let mut val = String::new();
        for word in words{
            if cnt == 0 {
                key = word.to_string();
            } else {
                val = word.to_string();
            }
            cnt+=1;
        }
        files.insert(key, val);
    }

    // get snapshot count
    let paths = fs::read_dir("./.geet/snapshots").unwrap();
    let mut cur = paths.count();
    let mut diff = false;

    // process files, create snapshots
    let mut newfiles: HashMap<String, String> = files.clone();
    for (pathname, snapshot) in files{
        if snapshot != "new" {continue;}
        diff = true;
        cur += 1;
        let name = cur.to_string();
        // copy file
        let mut pathtosnap = "./.geet/snapshots/".to_string();
        pathtosnap.push_str(&name);
        pathtosnap.push_str(".snp");
        let mut source = std::fs::File::open(&pathname)?;
        let mut target = std::fs::File::create(&pathtosnap)?;
        std::io::copy(&mut source, &mut target)?;
        // update map
        *newfiles.get_mut(&pathname).unwrap() = name;
    }

    if !diff {
        println!("No changes found");
        return Ok(());
    }

    // get commit count
    let paths = fs::read_dir("./.geet/commits").unwrap();
    let curhead = paths.count() + 1;

    // create commit file
    let mut buf = String::new();
    for (key, val) in newfiles{
        if key.len()==0 {continue};
        buf.push_str(&key);
        buf.push_str(" ");
        buf.push_str(&val);
        buf.push_str("\n");
    }
    let mut commitname = "./.geet/commits/".to_string();
    commitname.push_str(&curhead.to_string());
    commitname.push_str(".cmt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&commitname)
        .unwrap();
    file.write_all(buf.as_bytes())?;

    // update head.log
    let mut buf = curhead.to_string();
    buf.push_str(" ");
    buf.push_str(&head);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./.geet/head.log")
        .unwrap();
    file.write_all(buf.as_bytes())?;

    // update history.log
    buf.push_str(" ");
    buf.push_str(&msg);
    buf.push_str(" ");
    let curtime = chrono::Utc::now().to_string().replace(" ","%20%");
    buf.push_str(&curtime);
    buf.push_str("\n");
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./.geet/history.log")
        .unwrap();
    file.write_all(buf.as_bytes())?;

    // temp deletion done from main
    println!("Commit successful");
    Ok(())
}