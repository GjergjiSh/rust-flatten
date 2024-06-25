#![allow(dead_code)]

use std::sync::{Mutex, Once};

use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTRY_INSTANCE: Mutex<Registry> = Mutex::new(Registry {
        characteristics: Vec::new(),
    });
}

#[derive(Debug)]
struct Registry {
    characteristics: Vec<Characteristic>,
}

impl Registry {
    // Method to add a characteristic to the global instance of Registry
    pub fn add_characteristic(characteristic: Characteristic) {
        // Lock the mutex to access the Registry
        let mut registry_lock = REGISTRY_INSTANCE.lock().unwrap();
        registry_lock.characteristics.push(characteristic);
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
    pub characteristic_type: CharacteristicType
}

#[derive(Debug)]
pub enum CharacteristicType {
    MAP,
    CURVE,
    VALUE
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

impl_flatten_for_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, bool, char, String);

impl<T, const N: usize> Flatten for [T; N] {
}

impl<T1, T2> Flatten for (T1, T2) {}