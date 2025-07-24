use mlua::prelude::*;
use std::path::Path;
use url::Url;
use super::series::{ Series, Episode };

pub fn lua_run(path: &Path, keyword: &str) -> Result<Series, mlua::Error> {
    let lua = Lua::new();
    // add some functions for Lua
    lua.load(path).exec()?;
    let main: mlua::Function = lua.globals().get("main")?;
    let table: mlua::Table = main.call(keyword)?;
    let name: String = table.get("name")?;
    let image: String = table.get("image")?;
    let image: Url = Url::parse(&image).unwrap();
    let description: String = table.get("description")?;
    let lua_episodes: mlua::Table = table.get("episodes")?;
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
    Ok(series)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::super::run::lua_run;
    
    #[test]
    fn test_lua_run() {
        let path = Path::new("./tests/config_lua/simple-main.lua");
        let series = lua_run(&path, "mykey").unwrap();
        assert_eq!(series.name, "video_namemykey");
        assert_eq!(series.description, "description");
        assert_eq!(series.image.to_string(), "http://localhost/simple.png");
        assert_eq!(series.episodes.len(), 2);
        assert_eq!(series.episodes.get(0).unwrap().name, "1");
        assert_eq!(series.episodes.get(0).unwrap().addr.to_string(), "http://localhost/simple1.mp4");
        assert_eq!(series.episodes.get(1).unwrap().name, "2");
        assert_eq!(series.episodes.get(1).unwrap().addr.to_string(), "http://localhost/simple2.mp4");
    }
}
