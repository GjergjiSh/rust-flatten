#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use a2l_items::{Characteristic, CharacteristicType};
use flatten::Flatten;
use flatten_derive::Flatten;

#[derive(Debug)]
struct Registry {
    characteristics: Vec<Characteristic>,
}

macro_rules! simple_dbg {
    ($val:expr) => {{
        let val = &$val; // Take a reference to avoid moving ownership
        println!(
            "{} = {:?} at {}:{}",
            stringify!($val),
            val,
            file!(),
            line!()
        );
        val
    }};
}

macro_rules! named_a2l_flatten {
    ($var:ident) => {{
        let mut characteristics = $var.a2l_flatten().unwrap();
        // dbg!(&characteristics);
        for characteristic in &mut characteristics {
            characteristic.name = format!("{}.{}", stringify!($var), characteristic.name);
            dbg!(&characteristic.name);
        }
        characteristics
    }};
}

/* impl Registry {
    fn add_segment<T>(&mut self, segment: &T)
    where
        T: Flatten,
    {
            self.characteristics.extend(named_a2l_flatten!(segment));
    }
} */

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

#[derive(Flatten, Debug)]
struct Parent {
    #[comment = "Unique identifier"]
    #[min = 10]
    #[max = 20]
    #[unit = "unit"]
    uid: u32,
    child: Child,
    example_tuple: (i32, &'static str),
    array: [f32; 16],
    map: [[i32; 9]; 1],
    ndim_array: [[[i32; 4]; 1]; 2],
}

#[derive(Clone, Copy, Debug, Flatten)]
struct Child {
    uid: u32,
}

// macro_rules! named_a2l_flatten {
//     ($var:ident) => {{
//         let mut characteristics = $var.a2l_flatten();
//         for characteristic in &mut characteristics {
//             characteristic.name = format!("{}.{}", stringify!($var), characteristic.name);
//         }
//         characteristics
//     }};
// }

const PARENT: Parent = Parent {
    uid: 1,
    child: Child { uid: 2 },
    example_tuple: (3, "example"),
    array: [
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5,
    ],
    map: [[0, 0, 0, 0, 0, 0, 0, 1, 2]],
    ndim_array: [[[1, 2, 3, 4]], [[13, 14, 15, 16]]],
};

fn main() {
    

    let registry = &mut Registry {
        characteristics: Vec::new(),
    };

    registry.add_segment(&PARENT);

    // named_a2l_flatten!(parent);
    dbg!(registry);

    // for characteristic in named_a2l_flatten!(parent) {
    //     println!("{:?}", characteristic);
    // }

    // simple_dbg!(&parent);
}
