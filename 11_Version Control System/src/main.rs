use std::env;
use std::fs;

mod init;
mod commit;
mod add;
mod log;
mod head;

fn main(){
    let args: Vec<String> = env::args().collect();
    let cnt = args.len();
    if cnt<2{
        println!("Usage: geet init, geet add, geet remove, geet commit {{msg}}, geet log, geet head <version-id>");
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
    } else if args[1] == "commit" && cnt <= 3 {
        let mut msg = "No commit msg".to_string();
        if cnt == 3 {msg = args[2].clone();}
        let newmsg = msg.replace(" ", "%20%");
        if flag{
            commit::commit(&newmsg).expect("Failed to commit");
            add::remove().expect("Failed to remove staged changes");
        } else {
            println!("Not a geet repository");
        }
    } else if args[1] == "add" && cnt == 2 {
        if flag{
            add::add().expect("Failed to stage changes");
        } else {
            println!("Not a geet repository");
        }
    } else if args[1] == "remove" && cnt == 2 {
        if flag{
            add::remove().expect("Failed to remove changes");
            println!("Staged changes removed");
        } else {
            println!("Not a geet repository");
        }
    } else if args[1] == "log" && cnt == 2 {
        if flag{
            log::log().expect("Log corrupted");
        } else {
            println!("Not a geet repository");
        }
    } else if args[1] == "head" && cnt == 3 {
        if flag{
            head::head(&args[2]).expect("Version not found");
        } else {
            println!("Not a geet repository");
        }
    } else {
        println!("Unknown command");
    }
}