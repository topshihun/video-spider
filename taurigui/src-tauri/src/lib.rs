use dirs;
use luacore::get_detail as lua_get_detail;
use luacore::search as lua_search;
use luacore::Output;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter};
use tokio::task;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Series {
    id: usize,
    title: String,
    description: String,
    image_url: String,
    episodes: Vec<Episode>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Episode {
    name: String,
    url: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchError {
    from_path: String,
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn search(app_handle: AppHandle, keyword: &str) -> Result<(), ()> {
    static ID: AtomicUsize = AtomicUsize::new(0);
    println!("searching for {}", keyword);
    let lua_paths = get_luapaths();
    println!("lua_paths: {:?}", lua_paths);
    for lua_path in lua_paths {
        let lua_path_clone = lua_path.clone();
        let key_word_clone = keyword.to_string();
        let datas = match task::spawn_blocking(move || {
            lua_search(&lua_path_clone, &key_word_clone, Some(Output::stdout()))
        })
        .await
        .unwrap()
        {
            Ok(o) => o,
            Err(e) => {
                println!("emit error: {}", e);
                app_handle
                    .emit(
                        "search_error",
                        SearchError {
                            from_path: lua_path.to_string_lossy().to_string(),
                            message: e.to_string(),
                        },
                    )
                    .unwrap();
                continue;
            }
        };
        for data in datas {
            let lua_path_clone = lua_path.clone();
            let data_clone = data.clone();
            let series = match task::spawn_blocking(move || {
                lua_get_detail(&lua_path_clone, &data_clone, Some(Output::stdout()))
            })
            .await
            .unwrap()
            {
                Ok(o) => o,
                Err(e) => {
                    println!("emit error: {}", e);
                    app_handle
                        .emit(
                            "search_error",
                            SearchError {
                                from_path: lua_path.to_string_lossy().to_string(),
                                message: e.to_string(),
                            },
                        )
                        .unwrap();
                    break;
                }
            };
            println!("emit series:\n{:?}", series);
            let mut episodes = Vec::new();
            for episode in series.episodes {
                episodes.push(Episode {
                    name: episode.name,
                    url: episode.addr.to_string(),
                });
            }
            app_handle
                .emit(
                    "search_result",
                    Series {
                        id: ID.load(Ordering::Relaxed),
                        title: series.name,
                        description: series.description,
                        image_url: series.image.to_string(),
                        episodes,
                    },
                )
                .unwrap();
            ID.fetch_add(1, Ordering::Relaxed);
        }
    }
    Ok(())
}

// get all lua paths from config directory
fn get_luapaths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let config_path = dirs::config_dir().unwrap();
    let config_path = config_path.join("videospider");
    println!("config path: {}", config_path.to_str().unwrap());
    for entry_result in fs::read_dir(config_path).unwrap() {
        let entry = entry_result.unwrap();
        let path = entry.path();
        println!("path: {}", path.to_str().unwrap());
        if path.is_file() {
            if path.extension().unwrap_or_default() == "lua" {
                paths.push(path);
                println!("is lua file");
            }
        }
    }
    paths
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
