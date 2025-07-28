use mlua::{ Lua, Table, Result, Value::* };
use urlencoding::{ encode, decode };
use serde_json::Value;
use std::string::String;

// Lua function
fn http_get(_: &Lua, url: String) -> Result<String> {
    let body = reqwest::blocking::get(url)
        // TODO: result
        .unwrap()
        .text()
        .unwrap();
    Ok(body)
}

fn json_to_table(lua: &Lua, table: &Table, key: Option<&str>, value: &Value) -> Result<()> {
    match value {
        Value::Null => {
            if let Some(k) = key {
                table.set(k, Nil)?;
            } else {
                table.push(Nil)?;
            }
        },
        Value::Bool(b) => {
            if let Some(k) = key {
                table.set(k, Boolean(*b))?;
            } else {
                table.push(Boolean(*b))?;
            }
        },
        Value::Number(n) => {
            if let Some(k) = key {
                table.set(k, n.as_f64().unwrap())?;
            } else {
                table.push(Number(n.as_f64().unwrap()))?;
            }
        },
        Value::String(s) => {
            if let Some(k) = key {
                table.set(k, s.clone())?;
            } else {
                table.push(s.clone())?;
            }
        },
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
        },
        Value::Object(o) => {
            if let Some(k) = key {
                let map = lua.create_table()?;
                for (k, v) in o {
                    json_to_table(lua, &map, Some(k), v)?;
                }
                table.set(k, map)?;
            } else {
                for (k, v) in o {
                    json_to_table(lua, table, Some(k), v)?;
                }
            }
        },
    }
    Ok(())
}

// Lua function
fn json_parse(lua: &Lua, data: String) -> Result<Table> {
    let table = lua.create_table()?;
    let json: Value = serde_json::from_str(&data).expect("json parse failed");
    json_to_table(lua, &table, Option::None, &json)?;
    Ok(table)
}

// Lua function
fn string_split(lua: &Lua, (data, split_str): (String, String)) -> Result<Table> {
    let split: Vec<&str> = data.split(&split_str).collect();
    let table = lua.create_table()?;
    for v in split {
        table.push(v.to_string())?;
    }
    Ok(table)
}

// Lua function
fn url_encode(_: &Lua, data: String) -> Result<String> {
    Ok(encode(&data).to_string())
}

// Lua function
fn url_decode(_: &Lua, data: String) -> Result<String> {
    Ok(decode(&data).expect("data is not utf-8").to_string())
}

pub fn lua_extension(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let utils = lua.create_table()?;

    utils.set("http_get", lua.create_function(http_get)?)?;
    utils.set("json_parse", lua.create_function(json_parse)?)?;
    utils.set("string_split", lua.create_function(string_split)?)?;
    utils.set("url_encode", lua.create_function(url_encode)?)?;
    utils.set("url_decode", lua.create_function(url_decode)?)?;

    globals.set("utils", utils)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        json_parse,
        string_split,
        url_encode,
        url_decode,
        lua_extension,
    };
    use mlua::{Lua, Table};

    #[test]
    // Just check usage of these functions call
    fn test_lua_extension() {
        let lua = Lua::new();
        lua_extension(&lua).unwrap();
        lua.load(r#"
            http_get_res = utils.http_get("https://bing.com")
            json_parse_res = utils.json_parse('{"test": "test"}')
            string_split_res = utils.string_split("test=test=test", "=")
            url_encode = utils.url_encode("1=1&2=2")
            url_decode = utils.url_decode("1%3D1%262%3D2")
            "#)
            .exec()
            .unwrap();
    }

    #[test]
    fn test_json_parse() {
        let lua = Lua::new();
        lua.globals()
            .set("json_parse", lua.create_function(json_parse).unwrap())
            .unwrap();
        lua.load(r#"
            rose = json_parse('{"name": "Rose", "age": 18, "hobby": ["computer game", "gardening"]}')
            "#)
            .exec()
            .unwrap();

        let rose: Table = lua.globals().get("rose").unwrap();
        assert_eq!(rose.get::<String>("name").unwrap(), "Rose");
        assert_eq!(rose.get::<i32>("age").unwrap(), 18);
        let hobby: Table = rose.get("hobby").unwrap();
        assert_eq!(hobby.len().unwrap(), 2);
        assert_eq!(hobby.get::<String>(1).unwrap(), "computer game");
        assert_eq!(hobby.get::<String>(2).unwrap(), "gardening");
    }

    #[test]
    fn test_string_split() {
        let lua = Lua::new();
        lua.globals()
            .set("string_split", lua.create_function(string_split).unwrap())
            .unwrap();
        lua.load(r#"
            res = string_split("name:myname:yourname", ":")
            "#)
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
        lua.load(r#"
            res = url_encode("ac=b&b=ac")
            "#)
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
        lua.load(r#"
            res = url_decode("ac%3Db%26b%3Dac")
            "#)
            .exec()
            .unwrap();
        let res: String = lua.globals().get("res").unwrap();
        assert_eq!(res, "ac=b&b=ac");
    }
}
