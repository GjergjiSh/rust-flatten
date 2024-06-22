#![allow(unused)]
#![allow(non_snake_case)]

struct Parent {
    uid: u32,
    child: Child,
}

#[derive(Debug)]
struct Child {
    uid: u32,
}

struct FlattenedParent {
    uid: u32,
    Child_uid: u32,
}

impl Parent {
    // Method to flatten the Parent struct
    fn flatten(&self) -> FlattenedParent {
        FlattenedParent {
            uid: self.uid,
            Child_uid: self.child.uid,
        }
    }
}


fn main() {
    let parent = Parent {
        uid: 1,
        child: Child { uid: 2 },
    };

    let flattened_parent = parent.flatten();
    println!("FlattenedParent {{ uid: {}, Child_uid: {} }}", flattened_parent.uid, flattened_parent.Child_uid);
}
