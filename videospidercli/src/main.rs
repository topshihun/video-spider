use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use videospider::Output;
use videospider::get_config_path;
use videospider::get_lua_files;
use videospider::luafiles::LuaFile;
use videospider::search::{SearchMessage, search};

#[derive(Parser)]
#[command(version, about = "A test videospider tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all lua files in configuration path.
    Show,
    /// Search keyword by lua.
    Search {
        /// lua path
        lua_path: PathBuf,
        /// keyword
        keyword: String,
    },
}

fn main() {
    let command = Cli::parse().command;
    match command {
        Commands::Show => {
            let lua_file_list = get_lua_files();
            println!(
                "The number of lua files: {} in {}",
                lua_file_list.len(),
                get_config_path().to_string_lossy()
            );
            for lua_file in lua_file_list {
                println!(
                    "name: {} \tpath: {}",
                    lua_file.name,
                    lua_file.path.to_string_lossy()
                );
            }
        }
        Commands::Search { lua_path, keyword } => {
            let lua_file = LuaFile {
                name: lua_path.file_name().unwrap().to_string_lossy().into_owned(),
                path: lua_path,
            };
            let (sender, receiver) = channel::<SearchMessage>();
            search(sender, &[lua_file], &keyword, Some(Output::stdout()));
            while let SearchMessage::Continue(lua_file, res) = receiver.recv().unwrap() {
                let series_list = match res {
                    Ok(o) => o,
                    Err(e) => {
                        println!("{} error occurred:", lua_file.name);
                        println!("{}", e);
                        return;
                    },
                };
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
                    println!("================================");
                }
            }
        }
    }
}
