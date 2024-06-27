pub mod prelude {
    pub use crate::{
        Characteristic, CharacteristicContainer, CharacteristicType,
    };
}


pub trait CharacteristicContainer {
    fn characteristics(&self) -> Option<Vec<Characteristic>> {
        None
    }
}

#[derive(Debug)]
pub struct Characteristic {
    name: String,
    datatype: String,
    comment: String,
    min: i64,
    max: i64,
    unit: String,
    characteristic_type: CharacteristicType,
    offset: u16,
    extension: u8,
}

impl Characteristic {
    pub fn new(
        name: String,
        datatype: String,
        comment: String,
        min: i64,
        max: i64,
        unit: String,
        characteristic_type: CharacteristicType,
        offset: u16,
        extension: u8,
    ) -> Self {
        Characteristic {
            name,
            datatype,
            comment,
            min,
            max,
            unit,
            characteristic_type,
            offset,
            extension,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn datatype(&self) -> &String {
        &self.datatype
    }
    pub fn comment(&self) -> &String {
        &self.comment
    }
    pub fn min(&self) -> &i64 {
        &self.min
    }
    pub fn max(&self) -> &i64 {
        &self.max
    }
    pub fn unit(&self) -> &String {
        &self.unit
    }
    pub fn characteristic_type(&self) -> &CharacteristicType {
        &self.characteristic_type
    }
    pub fn offset(&self) -> &u16 {
        &self.offset
    }
    pub fn extension(&self) -> &u8 {
        &self.extension
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug)]
pub enum CharacteristicType {
    MAP,
    CURVE,
    VALUE,
}

// The CharacteristicContainer trait implementation for Rust primitives
// is simply a blanket (empty) trait implementation. This macro is used
// to automatically generate the implementation for Rust primitives
macro_rules! impl_flatten_for_primitive {
    ($($t:ty),*) => {
        $(
            impl CharacteristicContainer for $t {}
        )*
    };
}

impl_flatten_for_primitive!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, bool, char, String
);

// The implementation of the CharacteristicContainer trait for
// arrays is also a blanket (empty) trait implementation
impl<T, const N: usize> CharacteristicContainer for [T; N] {}
