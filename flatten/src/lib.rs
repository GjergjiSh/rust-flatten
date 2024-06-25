#![allow(dead_code)]

#[derive(Debug)]
struct Registry {
    characteristics: Vec<Characteristic>,
}

impl Registry {
    fn add_segment<T>(&mut self, segment: &T)
    where
        T: Flatten,
    {
        if let Some(characteristics) = segment.a2l_flatten() {
            self.characteristics.extend(characteristics.into_iter());
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