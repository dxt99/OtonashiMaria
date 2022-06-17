use std::collections::{HashMap, LinkedList};
use std::fs;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};

// traverses directory (bfs) and returns every non geet file
fn traverse() -> std::io::Result<HashMap<String, String>>{
    let mut files: HashMap<String, String> = HashMap::new();
    let mut nodes: LinkedList<String> = LinkedList::new();

    nodes.push_back("./".to_string());
    while nodes.len()>0{
        let tarpath = nodes.pop_front().unwrap();
        if tarpath == "./.geet".to_string() {continue;}
        if tarpath == "./target".to_string() {continue;}   //TODO: remove this later pls
        let paths = fs::read_dir(tarpath).unwrap();
        for path in paths{
            let pathtemp = path.unwrap();
            let filetype = pathtemp.file_type().unwrap();
            let temp = pathtemp.path().to_str().unwrap().to_string();
            if filetype.is_dir() {nodes.push_back(temp);}
            else {files.insert(temp, "new".to_string());}
        }
    }
    
    Ok(files)
}

fn issamefile(patha: &String, pathb: &String) ->  std::io::Result<bool>{
    let a = fs::File::open(patha)?;
    let b = fs::File::open(pathb)?;
    
    if a.metadata().unwrap().len() != b.metadata().unwrap().len() {
        return Ok(false);
    }

    let abuf= BufReader::new(a);
    let bbuf = BufReader::new(b);

    for (b1, b2) in abuf.bytes().zip(bbuf.bytes()) {
        if b1.unwrap() != b2.unwrap() {
            return Ok(false);
        }
    }
    Ok(true)
}

// maps every file in the directory to a number
pub fn add() -> std::io::Result<()>{
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

    //get parent map
    let mut files: HashMap<String, String> = HashMap::new();
    if head!="0"{
        let mut commitname = "./.geet/commits/".to_string();
        commitname.push_str(&head);
        commitname.push_str(".cmt");
        let mut commitlog = fs::File::open(commitname)?;
        let mut commitlogtext = String::new();
        commitlog.read_to_string(&mut commitlogtext)?;

        let commitlines = commitlogtext.split("\n");
        for line in commitlines{
            let words = line.split(" ");
            let mut cnt = 0;
            let mut pathname = String::new();
            let mut snapshot = String::new();
            for word in words{
                if cnt == 0 {pathname = word.to_string();}
                else {snapshot = word.to_string();}
                cnt+=1;
            }
            files.insert(pathname, snapshot);
        }

    }

    // get head files
    let mut newfiles = traverse().unwrap();
    
    for (pathname, snapshot) in files {
        if newfiles.contains_key(&pathname) {
            // check if file is the same
            let mut snappath = "./.geet/snapshots/".to_string();
            snappath.push_str(&snapshot);
            snappath.push_str(".snp");
            if issamefile(&pathname, &snappath).unwrap() {
                *newfiles.get_mut(&pathname).unwrap() = snapshot;
            }
        }
    }
    //writing to temp
    let mut temp = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./.geet/temp.log")
        .unwrap();

    let mut buffer = String::new();
    for (pathname, snapshot) in newfiles {
        buffer.push_str(&pathname);
        buffer.push_str(" ");
        buffer.push_str(&snapshot);
        buffer.push_str("\n");
    }
    temp.write_all(buffer.as_bytes())?;

    println!("Changes staged");
    Ok(())
}

pub fn remove()-> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("./.geet/temp.log")
        .unwrap();
    file.write_all(b"")?;

    Ok(())
}