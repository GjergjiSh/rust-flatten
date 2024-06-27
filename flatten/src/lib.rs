#![allow(dead_code)]
#![allow(unused_imports)]

use lazy_static::lazy_static;
use std::sync::{Mutex, Once};

// Assuming a fixed size for the array
const MAX_CHARACTERISTICS: usize = 10;

// Global instance without lazy_static
static REGISTRY_INSTANCE: Mutex<Registry> = Mutex::new(Registry {
    characteristics: [None, MAX_CHARACTERISTICS], // Array of Option<Characteristic>, assuming Characteristic is Clone
});

#[derive(Debug)]
pub struct Registry {
    characteristics: [Characteristic; MAX_CHARACTERISTICS],
}

impl Registry {
    // Method to add a characteristic to the global instance of Registry
    pub fn add_characteristic(characteristic: Characteristic) {
        // Lock the mutex to access the Registry
        let mut registry_lock = REGISTRY_INSTANCE.lock().unwrap();
        registry_lock.characteristics.push(characteristic);
    }
}

#[derive(Debug, Clone)]
pub struct Characteristic {
    pub name: String,
    pub datatype: String,
    pub comment: String,
    pub min: i64,
    pub max: i64,
    pub unit: String,
    pub characteristic_type: CharacteristicType
}

#[derive(Debug, Clone)]
pub enum CharacteristicType {
    MAP,
    CURVE,
    VALUE
}

pub trait Flatten {
    fn a2l_flatten(&self) -> Vec<Characteristic>;

    fn a2l_flatten_optional(&self) -> Option<Vec<Characteristic>> {
        None
    }
}

// Helper to define blanket implementations of
// the Flatten trait for all rust primitive types
macro_rules! impl_flatten_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Flatten for $t {
                fn a2l_flatten(&self) -> Vec<Characteristic> {
                    panic!("Not implemented for type {}", stringify!($t));
                }

                fn a2l_flatten_optional(&self) -> Option<Vec<Characteristic>> {
                    None
                }
            }
        )*
    };
}

impl_flatten_for_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, bool, char, String);

//TODO: Error messages
impl<T, const N: usize> Flatten for [T; N] {
    fn a2l_flatten(&self) -> Vec<Characteristic> {
        panic!("Not implemented for type [T; N]");
    }
}

//TODO: Error messages
impl<T1, T2> Flatten for (T1, T2) {
    fn a2l_flatten(&self) -> Vec<Characteristic> {
        panic!("Not implemented for type (T1, T2)");
    }
}