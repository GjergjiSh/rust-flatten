#![allow(unused)]
#![allow(non_snake_case)]

use flatten::Flatten;
use flatten_derive::Flatten;
use a2l_items::Characteristic;

#[derive(Flatten)]
struct Parent {
    // uid: u32,
    // name: String,
    child: Child,
}

#[derive(Clone, Copy, Debug, Flatten)]
struct Child {
    uid: u32,
}

fn main() {
    let parent = Parent {
        // uid: 1,
        // name: "Billy".to_string(),
        child: Child { uid: 2 },
    };

    let x = parent.a2l_flatten();
    dbg!(x);
}
