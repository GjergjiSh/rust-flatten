#![allow(dead_code)]

use std::sync::{Mutex, Once};
use std::collections::HashMap;

#[derive(Debug)]
struct Registry {
    characteristics: Vec<Characteristic>,
}

//TODO: Check if we can solve this with refs instead of moves
impl Registry {
    // Method to add a characteristic to the global instance of Registry
    fn add_characteristic(&mut self, characteristic: Characteristic) {
        self.characteristics.push(characteristic);
    }

    pub fn add_segment<T>(&mut self, segment: &T)
    where
        T: Flatten,
    {
        for characteristic in segment.a2l_flatten().unwrap() {
            //TODO: Setting the characteristic address here is ugly
            // let ext = 0; //XCP_ADDR_EXT_APP
            // let offset = ((&self.#field_name as *const _ as *const u8 as usize) - (self as *const _ as *const u8 as usize)) as u16;
            self.add_characteristic(characteristic)
        }
    }
}

#[derive(Debug)]
pub struct Characteristic {
    pub name: String,
    pub datatype: String,
    pub comment: String,
    pub min: i64,
    pub max: i64,
    pub unit: String,
    pub characteristic_type: CharacteristicType,
    //TODO: Might be unnecessary
    pub offset: u16,
    pub extension: u8
}

#[derive(Debug)]
pub enum CharacteristicType {
    MAP,
    CURVE,
    VALUE,
}

pub trait Flatten {
    fn a2l_flatten(&self) -> Option<Vec<Characteristic>> {
        None
    }
}

// Helper to define blanket implementations of
// the Flatten trait for all rust primitive types
macro_rules! impl_flatten_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Flatten for $t {}
        )*
    };
}

impl_flatten_for_primitive!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, bool, char, String
);

impl<T, const N: usize> Flatten for [T; N] {}

impl<T1, T2> Flatten for (T1, T2) {}
