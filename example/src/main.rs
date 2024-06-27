#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use flatten::{Characteristic, CharacteristicType, Flatten};
use flatten_derive::Flatten;

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

#[derive(Clone, Copy, Flatten, Debug)]
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

impl Parent {
    const fn make() -> Parent {
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

        PARENT
    }
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

const PARENT: Parent = Parent::make();

#[derive(Debug, Copy, Clone)]
struct CalPage<T: Copy + Clone>(usize, T);

struct CalSeg<T>
where
    T: Copy + Clone + Flatten,
{
    ecu_page: Box<CalPage<T>>,
    xcp_page: Arc<Mutex<CalPage<T>>>,
}

impl<T> CalSeg<T> where T: Copy + Clone + Flatten {
    pub fn serialize(&self) {
        dbg!(self.ecu_page.as_ref().1.a2l_flatten());
    }
}



fn main() {
    let chars = PARENT.a2l_flatten();
    // dbg!(chars);

    // Create an instance of CalPage using PARENT
    let ecu_page = Box::new(CalPage(0, PARENT));

    // Create an instance of CalSeg using CalPage instance
    let xcp_page = Arc::new(Mutex::new(CalPage(0, PARENT)));

    let cal_seg_instance = CalSeg {
        ecu_page: ecu_page,
        xcp_page: xcp_page,
    };

    cal_seg_instance.serialize();

    // let registry = &mut Registry {
    //     characteristics: Vec::new(),
    // };

    // registry.add_segment(&PARENT);

    // // named_a2l_flatten!(parent);
    // dbg!(registry);

    // for characteristic in named_a2l_flatten!(parent) {
    //     println!("{:?}", characteristic);
    // }

    // simple_dbg!(&parent);
}
