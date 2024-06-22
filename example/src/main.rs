#![allow(unused)]

struct Parent {
    uid: u32,
    child: Child,
}

#[derive(Debug)]
struct Child {
    uid: u32,
}

impl std::fmt::Debug for Parent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parent")
            .field("uid", &self.uid)
            .field("Child.uid", &self.child.uid)
            .finish()
    }
}

fn main() {
    let parent = Parent {
        uid: 1,
        child: Child { uid: 2 },
    };

    dbg!(parent);
}
