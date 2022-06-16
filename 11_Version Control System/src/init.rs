use std::fs;

pub fn init() -> std::io::Result<()>{
    fs::create_dir("./.geet/")?;
    fs::create_dir("./.geet/files")?;
    fs::create_dir("./.geet/commits")?;
    fs::File::create("./.geet/history.txt")?;
    fs::File::create("./.geet/temp.txt")?;
    Ok(())
}