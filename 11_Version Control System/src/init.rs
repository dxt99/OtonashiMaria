use std::fs;
/*
history.log
ID PARENT
ID PARENT
...
ID PARENT
HEAD

temp.log

 */
pub fn init() -> std::io::Result<()>{
    fs::create_dir("./.geet/")?;
    fs::create_dir("./.geet/snapshots")?;
    fs::create_dir("./.geet/commits")?;
    fs::File::create("./.geet/history.log")?;
    fs::File::create("./.geet/temp.log")?;
    Ok(())
}