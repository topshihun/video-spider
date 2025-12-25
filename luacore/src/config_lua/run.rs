use super::super::series::{Episode, Series};
use super::extension::lua_extension;
use super::output::Output;
use mlua::prelude::*;
use std::path::Path;
use url::Url;

pub fn lua_search(path: &Path, keyword: &str, output: Option<Output>) -> LuaResult<Vec<String>> {
    let lua = Lua::new();
    lua_extension(&lua, output)?;
    lua.load(path).exec()?;
    let search_function = lua.globals().get::<LuaFunction>("search")?;
    let datas = search_function.call::<LuaTable>(keyword)?;
    let mut result = Vec::new();
    for i in 1..=datas.len()? {
        let data = datas.get::<String>(i)?;
        result.push(data);
    }
    Ok(result)
}

pub fn lua_get_detail(path: &Path, data: &str, output: Option<Output>) -> LuaResult<Series> {
    let lua = Lua::new();
    lua_extension(&lua, output)?;
    lua.load(path).exec()?;
    let get_detail_function = lua.globals().get::<LuaFunction>("get_detail")?;
    let series = get_detail_function.call::<LuaTable>(data)?;
    let name = series.get("name")?;
    let description = series.get("description")?;
    let image = series.get::<String>("image")?;
    let image = Url::parse(&image).unwrap();
    let episodes_table = series.get::<LuaTable>("episodes")?;
    let mut episodes = Vec::new();
    for i in 1..=episodes_table.len()? {
        let episode = episodes_table.get::<LuaTable>(i)?;
        let name = episode.get::<String>("name")?;
        let addr = episode.get::<String>("addr")?;
        let addr = Url::parse(&addr).unwrap();
        episodes.push(Episode { name, addr });
    }
    Ok(Series {
        name,
        description,
        image,
        episodes,
    })
}

#[cfg(test)]
mod tests {
    use super::lua_get_detail;
    use super::lua_search;
    use std::path::Path;

    #[test]
    fn test_lua_search() {
        let path = Path::new("./tests/config_lua/simple_main.lua");
        let datas = lua_search(path, "key", None).unwrap();
        assert_eq!(datas.len(), 2);
        assert_eq!(datas.get(1).unwrap(), "This is a data2");
    }

    #[test]
    fn test_lua_get_detail() {
        let path = Path::new("./tests/config_lua/simple_main.lua");
        let series = lua_get_detail(path, "Data", None).unwrap();
        assert_eq!(series.name, "video_name");
        assert_eq!(series.description, "description");
        assert_eq!(series.image.as_str(), "http://localhost/simple.png");
        assert_eq!(series.episodes.len(), 2);
        assert_eq!(series.episodes.first().unwrap().name, "1");
        assert_eq!(
            series.episodes.first().unwrap().addr.as_str(),
            "http://localhost/simple1.mp4"
        );
        assert_eq!(series.episodes.get(1).unwrap().name, "2");
        assert_eq!(
            series.episodes.get(1).unwrap().addr.as_str(),
            "http://localhost/simple2.mp4"
        );
    }
}
