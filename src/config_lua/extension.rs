use super::output::Output;
use mlua::prelude::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{io::Write, string::String};
use urlencoding::{decode, encode};

use crate::utils::decode_unicode;

// Lua function
fn http_get(_: &Lua, url: String) -> LuaResult<String> {
    let body = Client::new()
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
        .send()
        .unwrap();
    if body.status().is_success() {
        return Ok(body.text().unwrap());
    }
    Err(LuaError::RuntimeError(body.text().unwrap()))
}

fn json_to_table(lua: &Lua, table: &LuaTable, key: Option<&str>, value: &Value) -> LuaResult<()> {
    match value {
        Value::Null => {
            if let Some(k) = key {
                table.set(k, LuaNil)?;
            } else {
                table.push(LuaNil)?;
            }
        }
        Value::Bool(b) => {
            if let Some(k) = key {
                table.set(k, *b)?;
            } else {
                table.push(*b)?;
            }
        }
        Value::Number(n) => {
            if let Some(k) = key {
                table.set(k, n.to_string())?;
            } else {
                table.push(n.to_string())?;
            }
        }
        Value::String(s) => {
            if let Some(k) = key {
                table.set(k, s.clone())?;
            } else {
                table.push(s.clone())?;
            }
        }
        Value::Array(a) => {
            let arr = lua.create_table()?;
            for v in a {
                json_to_table(lua, &arr, Option::None, v)?;
            }
            if let Some(k) = key {
                table.set(k, arr)?;
            } else {
                table.push(arr)?;
            }
        }
        Value::Object(o) => {
            let map = lua.create_table()?;
            for (k, v) in o {
                json_to_table(lua, &map, Some(k), v)?;
            }
            if let Some(k) = key {
                table.set(k, map)?;
            } else {
                table.push(map)?;
            }
        }
    }
    Ok(())
}

// Lua function
fn json_parse(lua: &Lua, data: String) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    let json: Value = serde_json::from_str(&data).expect("json parse failed");
    json_to_table(lua, &table, Option::None, &json)?;
    table.get::<LuaTable>(1)
}

// Lua function
fn string_split(lua: &Lua, (data, split_str): (String, String)) -> LuaResult<LuaTable> {
    let split: Vec<&str> = data.split(&split_str).collect();
    let table = lua.create_table()?;
    for v in split {
        table.push(v.to_string())?;
    }
    Ok(table)
}

// Lua function
fn url_encode(_: &Lua, data: String) -> LuaResult<String> {
    Ok(encode(&data).to_string())
}

// Lua function
fn url_decode(_: &Lua, data: String) -> LuaResult<String> {
    Ok(decode(&data).expect("data is not utf-8").to_string())
}

// Lua function
fn unicode_encode(_: &Lua, data: String) -> LuaResult<String> {
    let mut result = String::new();
    for c in data.chars() {
        result.push_str(&format!("\\u{{{:X}}}", c as u32));
    }
    Ok(result)
}

// Lua function
fn unicode_decode(_: &Lua, data: String) -> LuaResult<String> {
    Ok(decode_unicode(&data))
}

// Lua function
fn log(lua: &Lua, data: LuaVariadic<LuaValue>) -> LuaResult<()> {
    let tostring = lua.globals().get::<LuaFunction>("tostring")?;
    let mut output: mlua::AppDataRefMut<'_, Output> = lua.app_data_mut().expect("check app data");
    for value in data.iter() {
        let string: String = tostring.call(value)?;
        write!(output, "{}", string)?;
    }
    Ok(())
}

// Lua function
fn nothing_log(_: &Lua, _: LuaVariadic<LuaValue>) -> LuaResult<()> {
    Ok(())
}

pub fn lua_extension(lua: &Lua, output: Option<Output>) -> LuaResult<()> {
    let globals = lua.globals();
    let utils = lua.create_table()?;

    utils.set("http_get", lua.create_function(http_get)?)?;
    utils.set("json_parse", lua.create_function(json_parse)?)?;
    utils.set("string_split", lua.create_function(string_split)?)?;
    utils.set("url_encode", lua.create_function(url_encode)?)?;
    utils.set("url_decode", lua.create_function(url_decode)?)?;
    utils.set("unicode_encode", lua.create_function(unicode_encode)?)?;
    utils.set("unicode_decode", lua.create_function(unicode_decode)?)?;

    utils.set(
        "log",
        lua.create_function(if output.is_none() { nothing_log } else { log })?,
    )?;

    // move output
    if let Some(output) = output {
        lua.set_app_data(output);
    }

    globals.set("utils", utils)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        path::Path,
        sync::{Arc, Mutex},
    };

    use super::{
        super::output::Output, json_parse, log, lua_extension, string_split, unicode_decode,
        unicode_encode, url_decode, url_encode,
    };
    use mlua::{Lua, Table};

    #[test]
    // Just check usage of these functions call
    fn test_lua_extension() {
        let lua = Lua::new();
        lua_extension(&lua, Some(Output::stdout())).unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_lua_extension.lua",
        ))
        .exec()
        .unwrap();
    }

    #[test]
    fn test_json_parse() {
        let lua = Lua::new();
        lua.globals()
            .set("json_parse", lua.create_function(json_parse).unwrap())
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_json_parse.lua",
        ))
        .exec()
        .unwrap();

        let rose: Table = lua.globals().get("rose").unwrap();
        assert_eq!(rose.get::<String>("name").unwrap(), "Rose");
        assert_eq!(rose.get::<i32>("age").unwrap(), 18);
        let hobby: Table = rose.get("hobby").unwrap();
        assert_eq!(hobby.len().unwrap(), 2);
        assert_eq!(hobby.get::<String>(1).unwrap(), "computer game");
        assert_eq!(hobby.get::<String>(2).unwrap(), "gardening");
        let classes: Table = rose.get("classes").unwrap();
        assert_eq!(classes.len().unwrap(), 2);
        assert_eq!(
            classes
                .get::<Table>(1)
                .unwrap()
                .get::<String>("name")
                .unwrap(),
            "math"
        );
        assert_eq!(
            classes
                .get::<Table>(1)
                .unwrap()
                .get::<String>("teacher")
                .unwrap(),
            "Math"
        );
        assert_eq!(
            classes
                .get::<Table>(2)
                .unwrap()
                .get::<String>("name")
                .unwrap(),
            "english"
        );
        assert_eq!(
            classes
                .get::<Table>(2)
                .unwrap()
                .get::<String>("teacher")
                .unwrap(),
            "English"
        );
    }

    #[test]
    fn test_string_split() {
        let lua = Lua::new();
        lua.globals()
            .set("string_split", lua.create_function(string_split).unwrap())
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_string_split.lua",
        ))
        .exec()
        .unwrap();
        let res: Table = lua.globals().get("res").unwrap();
        assert_eq!(res.len().unwrap(), 3);
        assert_eq!(res.get::<String>(1).unwrap(), "name");
        assert_eq!(res.get::<String>(2).unwrap(), "myname");
        assert_eq!(res.get::<String>(3).unwrap(), "yourname");
    }

    #[test]
    fn test_url_encode() {
        let lua = Lua::new();
        lua.globals()
            .set("url_encode", lua.create_function(url_encode).unwrap())
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_url_encode.lua",
        ))
        .exec()
        .unwrap();
        let res: String = lua.globals().get("res").unwrap();
        assert_eq!(res, "ac%3Db%26b%3Dac");
    }

    #[test]
    fn test_url_decode() {
        let lua = Lua::new();
        lua.globals()
            .set("url_decode", lua.create_function(url_decode).unwrap())
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_url_decode.lua",
        ))
        .exec()
        .unwrap();
        let res: String = lua.globals().get("res").unwrap();
        assert_eq!(res, "ac=b&b=ac");
    }

    #[test]
    fn test_unicode_encode() {
        let lua = Lua::new();
        lua.globals()
            .set(
                "unicode_encode",
                lua.create_function(unicode_encode).unwrap(),
            )
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_unicode_encode.lua",
        ))
        .exec()
        .unwrap();
        let res: String = lua.globals().get("res").unwrap();
        assert_eq!(
            res,
            "\\u{48}\\u{65}\\u{6C}\\u{6C}\\u{6F}\\u{20}\\u{4C}\\u{75}\\u{61}"
        );
    }

    #[test]
    fn test_unicode_decode() {
        let lua = Lua::new();
        lua.globals()
            .set(
                "unicode_decode",
                lua.create_function(unicode_decode).unwrap(),
            )
            .unwrap();
        lua.load(Path::new(
            "./tests/config_lua/extension/test_unicode_decode.lua",
        ))
        .exec()
        .unwrap();
        let res: String = lua.globals().get("res").unwrap();
        assert_eq!(res, "Hello Lua");
    }

    #[test]
    fn test_log() {
        let lua = Lua::new();
        let buffer = Arc::new(Mutex::new(Vec::new()));
        lua.set_app_data(Output::buffer(Arc::clone(&buffer)));
        lua.globals()
            .set("log", lua.create_function(log).unwrap())
            .unwrap();
        lua.load(Path::new("./tests/config_lua/extension/test_log.lua"))
            .exec()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&buffer.lock().unwrap()), "123");
    }
}
