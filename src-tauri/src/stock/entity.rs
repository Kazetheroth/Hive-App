use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType
}

#[derive(Serialize, Deserialize)]
pub struct EntityType {
    pub name: String
}

pub fn entity_from_string(str: &str) -> Entity {
    //println!("{}", str);
    let e: Entity = serde_json::from_str(str).unwrap();
    return e
}

pub fn stringify_entity(entity: &Entity) -> String {
    let e: String = serde_json::to_string(&entity).unwrap();
    return e
}

pub fn save(entity: &Entity) {
    let path = format!("data/stock/{}.json", entity.name);
    //let json = stringify_entity(entity);
    let result = serde_json::to_writer(&File::create(path).unwrap(), &entity);
}