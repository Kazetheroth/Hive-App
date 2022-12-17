use std::fs;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

pub fn create_dir_if_not_exist(path: &str) -> () {
    let _ = fs::create_dir_all(path);
    return;
}

pub fn read_entity_from_path(path: &str) -> &str {
    /*let entity_json = json!();
    let mut entity = {
        let text = fs::read_to_string(path).unwrap();
        serde_json::from_str::<Value>(&text).unwrap();
    };
    let entity_sz = entity;
    return entity_sz;*/
    return "aa";
}

pub fn create_new_entity() -> () {
    create_new_json_entity("", "");
    return;
}

fn create_new_json_entity(path: &str, json: &str) -> () {

    return;
}

fn create_new_image_entity() -> () {
    return;
}
