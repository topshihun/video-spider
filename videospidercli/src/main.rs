use clap::Parser;
use clap::Subcommand;
use luacore;
use luacore::Output;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "A test videospider tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search keyword by lua.
    Search {
        /// lua path
        lua_path: PathBuf,
        /// keyword
        keyword: String,
    },
    /// Get detail
    GetDetail {
        /// lua path
        lua_path: PathBuf,
        /// data
        data: String,
    },
}

fn main() {
    let command = Cli::parse().command;
    match command {
        Commands::Search { lua_path, keyword } => {
            let datas = match luacore::search(&lua_path, &keyword, Some(Output::stdout())) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("search failed:");
                    eprintln!("{}", e);
                    panic!();
                }
            };
            println!("keyword: {}", &keyword);
            for data in datas {
                println!("{}", data);
            }
        }
        Commands::GetDetail { lua_path, data } => {
            let series = match luacore::get_detail(&lua_path, &data, Some(Output::stdout())) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("get_detail failed:");
                    eprintln!("{}", e);
                    panic!();
                }
            };
            println!("data: {}", &data);
            println!("name: {}", series.name);
            println!("description: {}", series.description);
            println!("image: {}", series.image);
            for episode in series.episodes {
                println!("\tname: {}\turl:{}", episode.name, episode.addr);
            }
        }
    }
}
