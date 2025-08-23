use clap::Parser;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use videospider::Output;
use videospider::luafiles::LuaFile;
use videospider::search::{SearchMessage, search};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    keyword: String,

    #[arg(long)]
    lua_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let keyword = args.keyword;
    let lua_path = args.lua_path;
    let lua_file = LuaFile {
        name: lua_path.file_name().unwrap().to_string_lossy().into_owned(),
        path: lua_path,
    };
    let (sender, receiver) = channel::<SearchMessage>();
    search(sender, &[lua_file], &keyword, Some(Output::stdout()));
    while let SearchMessage::Continue(lua_file, res) = receiver.recv().unwrap() {
        let series_list = res.unwrap();
        println!("name: {}", lua_file.name);
        for series in series_list {
            println!("================================");
            println!("\tname: {}", series.name);
            println!("\tdescription: {}", series.description);
            println!("\timage: {}", series.image);
            println!("\tepisodes:");
            for episode in series.episodes {
                println!("\t\tname: {} url: {}", episode.name, episode.addr);
            }
        }
    }
}
