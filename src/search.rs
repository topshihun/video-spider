use mlua::prelude::*;
use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use threadpool::ThreadPool;
use super::series::Series;
use super::luafiles::LuaFile;
use super::error::{ Result, Error::LuaFailed };
use super::config_lua::run::lua_run;

const THREAD_NUM: usize = 10;

#[derive()]
pub enum SearchMessage {
    Continue(LuaFile, Result<Vec<Series>>),
    Finished,
}

pub fn search(sender: Sender<SearchMessage>, used_lua_files: &[LuaFile], keyword: &str) {
    let threadpool = ThreadPool::new(THREAD_NUM);
    let channel_valid = Arc::new(AtomicBool::new(true));
    for lua_file in used_lua_files {
        let sender = sender.clone();
        let keyword = keyword.to_string();
        let lua_file = lua_file.clone();
        let channel_valid = Arc::clone(&channel_valid);
        threadpool.execute(move || {
            if !channel_valid.load(Ordering::SeqCst) {
                return;
            }
            let res: LuaResult<Vec<Series>> = lua_run(&lua_file.path, &keyword);
            match res {
                Ok(series_list) => {
                    if sender.send(SearchMessage::Continue(lua_file, Ok(series_list))).is_err() {
                        channel_valid.store(false, Ordering::SeqCst);
                    }
                },
                Err(err) => {
                    if sender.send(SearchMessage::Continue(lua_file, Err(LuaFailed(err.to_string())))).is_err() {
                        channel_valid.store(false, Ordering::SeqCst);
                    }
                },
            }
        });
    }
    threadpool.join();
    if channel_valid.load(Ordering::SeqCst) {
        sender.send(SearchMessage::Finished).unwrap();
    }
}
