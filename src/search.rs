use std::sync::mpsc::Sender;
use super::config_lua::series::Series;
use super::luafiles::LuaFile;

pub fn search(sender: Sender<Series>, used_lua_files: &[LuaFile], keyword: &str) {
}
