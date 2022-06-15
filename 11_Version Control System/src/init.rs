use std::fs;

pub fn create_geet() -> std::io::Result<()>{
    fs::create_dir("./.geet/")?;
    fs::create_dir("./.geet/files")?;
    fs::create_dir("./.geet/commits")?;
    Ok(())
}

pub fn init(){
    let paths = fs::read_dir("./").unwrap();
    let names =
    paths.filter_map(|entry| {
        entry.ok().and_then(|e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
        )
    }).collect::<Vec<String>>();
    for name in names{
        if name == ".geet"{
            println!("Geet repository already initialized");
            return;
        }
    }
    create_geet().expect("Failed to create repository");
}