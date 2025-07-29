use mlua::prelude::*;
use std::path::Path;
use url::Url;
use super::series::{ Series, Episode };
use super::extension::lua_extension;

pub fn lua_run(path: &Path, keyword: &str) -> LuaResult<Vec<Series>> {
    let lua = Lua::new();
    // add some functions for Lua
    lua_extension(&lua)?;
    lua.load(path).exec()?;
    let main: LuaFunction = lua.globals().get("main")?;
    let series_list: LuaTable = main.call(keyword)?;
    let mut series_list_ret: Vec<Series> = Vec::new();
    for i in 1..=series_list.len()? {
        let table: LuaTable = series_list.get(i)?;
        let name: String = table.get("name")?;
        let image: String = table.get("image")?;
        let image: Url = Url::parse(&image).unwrap();
        let description: String = table.get("description")?;
        let lua_episodes: LuaTable = table.get("episodes")?;
        let mut episodes: Vec<Episode> = Vec::new();
        for pair in lua_episodes.pairs() {
            let (key, value): (String, String) = pair?;
            let value: Url = Url::parse(&value).unwrap();
            episodes.push(Episode {name: key, addr: value});
        }
        let series = Series {
            name,
            description,
            image,
            episodes,
        };
        series_list_ret.push(series);
    }
    Ok(series_list_ret)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::super::run::lua_run;
    
    #[test]
    fn test_lua_run() {
        let path = Path::new("./tests/config_lua/simple_main.lua");
        let series_list = lua_run(&path, "_mykey").unwrap();
        assert_eq!(series_list.len(), 2);

        assert_eq!(series_list.get(0).unwrap().name, "video_name1_mykey");
        assert_eq!(series_list.get(1).unwrap().name, "video_name2_mykey");

        assert_eq!(series_list.get(0).unwrap().description, "description");
        assert_eq!(series_list.get(1).unwrap().description, "description");

        assert_eq!(series_list.get(0).unwrap().image.to_string(), "http://localhost/simple.png");
        assert_eq!(series_list.get(1).unwrap().image.to_string(), "http://localhost/simple.png");

        assert_eq!(series_list.get(0).unwrap().episodes.len(), 2);
        assert_eq!(series_list.get(1).unwrap().episodes.len(), 2);

        assert_eq!(series_list.get(0).unwrap().episodes.get(0).unwrap().name, "1");
        assert_eq!(series_list.get(0).unwrap().episodes.get(0).unwrap().addr.to_string(), "http://localhost/simple1.mp4");
        assert_eq!(series_list.get(1).unwrap().episodes.get(0).unwrap().name, "1");
        assert_eq!(series_list.get(1).unwrap().episodes.get(0).unwrap().addr.to_string(), "http://localhost/simple1.mp4");

        assert_eq!(series_list.get(0).unwrap().episodes.get(1).unwrap().name, "2");
        assert_eq!(series_list.get(0).unwrap().episodes.get(1).unwrap().addr.to_string(), "http://localhost/simple2.mp4");
        assert_eq!(series_list.get(1).unwrap().episodes.get(1).unwrap().name, "2");
        assert_eq!(series_list.get(1).unwrap().episodes.get(1).unwrap().addr.to_string(), "http://localhost/simple2.mp4");
    }
}
