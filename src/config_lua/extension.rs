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
                json_to_table(lua, table, Option::None, v)?;
            }
            if let Some(k) = key {
                table.set(k, arr)?;
            } else {
                table.push(arr)?;
            }
        },
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
fn string_spilt(lua: &Lua, (data, split_str): (String, String)) -> Result<Table> {
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
    utils.set("string_spilt", lua.create_function(string_spilt)?)?;
    utils.set("url_encode", lua.create_function(url_encode)?)?;
    utils.set("url_decode", lua.create_function(url_decode)?)?;

    globals.set("utils", utils)?;

    Ok(())
}
