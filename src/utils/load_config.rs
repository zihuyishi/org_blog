use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use serde_json;
use serde_json::Value;

fn load_config() -> Result<serde_json::Value, io::Error> {
    let f = try!(File::open("config.json"));
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    try!(reader.read_to_string(&mut buffer));
    let data: serde_json::Value = serde_json::from_str(&buffer).unwrap();
    Ok(data)
}

pub fn value_by_key(key: &str) -> Option<serde_json::Value> {
    let config = load_config();
    if let Ok(json_value) = config {
        if let Value::Object(obj) = json_value {
            return obj.get(key).cloned();
        }
    }

    None
}

pub fn f64_by_key(key: &str) -> Option<f64> {
    let value = value_by_key(key);
    value.and_then(|f| {
        f.as_f64()
    })
}

pub fn i64_by_key(key: &str) -> Option<i64> {
    let value = value_by_key(key);
    value.and_then(|f| {
        f.as_i64()
    })
}

pub fn bool_by_key(key: &str) -> Option<bool> {
    let value = value_by_key(key);
    value.and_then(|f| {
        f.as_boolean()
    })
}

pub fn string_by_key(key: &str) -> Option<String> {
    let value = value_by_key(key);
    value.and_then(|f| {
        f.as_string().and_then(|s| Some(s.to_string()))
    })
}
