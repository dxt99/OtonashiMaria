use std::env;
use std::fs;

mod init;
mod commit;

fn main(){
    let args: Vec<String> = env::args().collect();
    let cnt = args.len();
    if cnt<2{
        println!("Usage: geet init, geet commit, geet log, geet head <version-id>");
        return;
    }
    let paths = fs::read_dir("./").unwrap();
    let names =
    paths.filter_map(|entry| {
        entry.ok().and_then(|e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
        )
    }).collect::<Vec<String>>();
    let mut flag = false;
    for name in names{
        if name == ".geet"{
            flag = true;
            break;
        }
    }
    if args[1] == "init" && cnt == 2 {
        if !flag{
            init::init().expect("Failed to create geet repository");
        } else {
            println!("Geet repository already initialized");
        }
    } else if args[1] == "commit" && cnt == 2 {
        if flag{
            commit::commit().expect("Failed to commit");
        } else {
            println!("Not a geet repository");
        }
    }
}