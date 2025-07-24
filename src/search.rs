use std::sync::mpsc::Sender;
use super::config_lua::series::Series;
use super::luafiles::LuaFile;

pub enum SearchMessage {
    Continue(Series),
    Finished,
}

pub fn search(sender: Sender<SearchMessage>, used_lua_files: &[LuaFile], keyword: &str) {
}
