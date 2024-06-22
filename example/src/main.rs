#![allow(unused)]
#![allow(non_snake_case)]

use flatten::Flatten;
use flatten_derive::Flatten;
use a2l_items::Characteristic;

#[derive(Debug)]
struct Registry {
    characteristics: Vec<Characteristic>,
}

impl Registry {
    fn add_segment<T>(&mut self, segment: T)
    where
        T: Flatten,
    {
        if let Some(characteristics) = segment.a2l_flatten() {
            self.characteristics.extend(characteristics.into_iter());
        }
    }
}

#[derive(Flatten)]
struct Parent {
    #[comment = "Unique identifier"]
    #[min = 10]
    #[max = 20]
    #[unit = "unit"]
    uid: u32,
    child: Child,
}

#[derive(Clone, Copy, Debug, Flatten)]
struct Child {
    uid: u32,
}

fn main() {
    let parent = Parent {
        uid: 1,
        child: Child { uid: 2 },
    };

    let registry = &mut Registry {
        characteristics: Vec::new(),
    };

    registry.add_segment(parent);
    dbg!(registry);
}
