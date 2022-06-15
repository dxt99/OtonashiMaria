use std::env;

mod init;

fn main(){
    let args: Vec<String> = env::args().collect();
    let cnt = args.len();
    if cnt<2{
        println!("Usage: geet init, geet commit, geet log, geet head <version-id>");
        return;
    }
    if args[1] == "init" && cnt == 2 {
        init::init();
    }
}