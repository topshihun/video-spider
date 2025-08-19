use super::super::series::{Episode, Series};
use super::extension::lua_extension;
use mlua::prelude::*;
use std::path::Path;
use url::Url;

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
        let lua_episode_list: LuaTable = table.get("episodes")?;
        let mut episodes: Vec<Episode> = Vec::new();
        for i in 1..=lua_episode_list.len()? {
            let lua_episode: LuaTable = lua_episode_list.get(i)?;
            let name: String = lua_episode.get("name")?;
            let addr: String = lua_episode.get("addr")?;
            let addr: Url = Url::parse(&addr).unwrap();
            episodes.push(Episode { name, addr });
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
    use super::super::run::lua_run;
    use std::path::Path;

    #[test]
    fn test_lua_run() {
        let path = Path::new("./tests/config_lua/simple_main.lua");
        let series_list = lua_run(path, "_mykey").unwrap();
        assert_eq!(series_list.len(), 2);

        assert_eq!(series_list.first().unwrap().name, "video_name1_mykey");
        assert_eq!(series_list.get(1).unwrap().name, "video_name2_mykey");

        assert_eq!(series_list.first().unwrap().description, "description");
        assert_eq!(series_list.get(1).unwrap().description, "description");

        assert_eq!(
            series_list.first().unwrap().image.to_string(),
            "http://localhost/simple.png"
        );
        assert_eq!(
            series_list.get(1).unwrap().image.to_string(),
            "http://localhost/simple.png"
        );

        assert_eq!(series_list.first().unwrap().episodes.len(), 2);
        assert_eq!(series_list.get(1).unwrap().episodes.len(), 2);

        assert_eq!(
            series_list.first().unwrap().episodes.first().unwrap().name,
            "1"
        );
        assert_eq!(
            series_list
                .first()
                .unwrap()
                .episodes
                .first()
                .unwrap()
                .addr
                .to_string(),
            "http://localhost/simple1.mp4"
        );
        assert_eq!(
            series_list.get(1).unwrap().episodes.first().unwrap().name,
            "1"
        );
        assert_eq!(
            series_list
                .get(1)
                .unwrap()
                .episodes
                .first()
                .unwrap()
                .addr
                .to_string(),
            "http://localhost/simple1.mp4"
        );

        assert_eq!(
            series_list.first().unwrap().episodes.get(1).unwrap().name,
            "2"
        );
        assert_eq!(
            series_list
                .first()
                .unwrap()
                .episodes
                .get(1)
                .unwrap()
                .addr
                .to_string(),
            "http://localhost/simple2.mp4"
        );
        assert_eq!(
            series_list.get(1).unwrap().episodes.get(1).unwrap().name,
            "2"
        );
        assert_eq!(
            series_list
                .get(1)
                .unwrap()
                .episodes
                .get(1)
                .unwrap()
                .addr
                .to_string(),
            "http://localhost/simple2.mp4"
        );
    }
}
