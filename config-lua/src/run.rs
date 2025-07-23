use mlua::prelude::*;
use std::path::Path;
use url::Url;
use crate::video::Video;

pub fn lua_run(path: &Path, keyword: &str) -> Result<Video, mlua::Error> {
    let lua = Lua::new();
    // add some functions for Lua
    lua.load(path).exec()?;
    let main: mlua::Function = lua.globals().get("main")?;
    let table: mlua::Table = main.call(keyword)?;
    let name: String = table.get("name")?;
    let image: String = table.get("image")?;
    let image: Url = Url::parse(&image).unwrap();
    let describtion: String = table.get("description")?;
    let lua_urls: mlua::Table = table.get("urls")?;
    let mut urls: Vec<Url> = Vec::new();
    for i in 1..=lua_urls.len().unwrap() {
        let url: String = lua_urls.get(i)?;
        let url: Url = Url::parse(&url).unwrap();
        urls.push(url);
    }
    let video = Video {
        name: name,
        description: describtion,
        image: image,
        urls: urls,
    };
    Ok(video)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::run::lua_run;
    
    #[test]
    fn test_lua_run() {
        let path = Path::new("../tests/config-lua/simple-main.lua");
        let video = lua_run(&path, "mykey").unwrap();
        assert_eq!(video.name, "video_namemykey");
        assert_eq!(video.description, "description");
        assert_eq!(video.image.to_string(), "http://localhost/simple.png");
        assert_eq!(2, video.urls.len());
        assert_eq!(video.urls.get(0).unwrap().to_string(), "http://localhost/simple1.mp4");
        assert_eq!(video.urls.get(1).unwrap().to_string(), "http://localhost/simple2.mp4");
    }
}
