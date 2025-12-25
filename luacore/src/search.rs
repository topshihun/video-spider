use super::Series;
use super::config_lua::run::lua_get_detail;
use super::config_lua::run::lua_search;
use super::error::{Error::LuaFailed, Result};
use crate::config_lua::output::Output;
use std::path::Path;

pub fn search(lua_file: &Path, keyword: &str, output: Option<Output>) -> Result<Vec<String>> {
    match lua_search(lua_file, keyword, output) {
        Ok(o) => Ok(o),
        Err(e) => Err(LuaFailed(e)),
    }
}

pub fn get_detail(lua_file: &Path, data: &str, output: Option<Output>) -> Result<Series> {
    match lua_get_detail(lua_file, data, output) {
        Ok(o) => Ok(o),
        Err(e) => Err(LuaFailed(e)),
    }
}
