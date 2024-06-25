#![allow(dead_code)]

#[derive(Debug)]
pub struct Characteristic {
    pub name: String,
    pub datatype: String,
    pub comment: String,
    pub min: i64,
    pub max: i64,
    pub unit: String,
    pub characteristic_type: CharacteristicType
}

#[derive(Debug)]
pub enum CharacteristicType {
    MAP,
    CURVE,
    VALUE
}