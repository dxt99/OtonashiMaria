use std::fs;
use std::io::prelude::*;
/*
history.log
ID PARENT datetime
ID PARENT datetime
ID PARENT datetime

temp.log and commit/ID.cmt
path fileID
path new
path fileID
path new

head.log
HEAD PARENT

snapshots
1.snp
2.snp
 */
pub fn init() -> std::io::Result<()>{
    fs::create_dir("./.geet/")?;
    fs::create_dir("./.geet/snapshots")?;
    fs::create_dir("./.geet/commits")?;
    fs::File::create("./.geet/history.log")?;
    fs::File::create("./.geet/temp.log")?;
    let mut file = fs::File::create("./.geet/head.log")?;
    file.write_all(b"0 0")?;
    Ok(())
}