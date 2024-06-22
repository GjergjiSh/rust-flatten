use a2l_items::Characteristic;

pub trait Flatten {
    fn a2l_flatten(&self) -> Vec<Characteristic>;
    //TODO: Why do we need this?
    fn to_a2l_optional(&self) -> Option<Vec<Characteristic>> { None }
}

impl Flatten for u32 {
    fn a2l_flatten(&self) -> Vec<Characteristic> {
        // vec![Characteristic {
        //     name: "".to_string(), // Primitive types do not have a name themselves
        //     datatype: "u32".to_string(),
        // }]
        panic!("a2l_flatten is a no-op for primitive types")
    }
}

impl Flatten for String {
    fn a2l_flatten(&self) -> Vec<Characteristic> {
        // vec![Characteristic {
        //     name: "".to_string(), // Primitive types do not have a name themselves
        //     datatype: "u32".to_string(),
        // }]
        panic!("a2l_flatten is a no-op for primitive types")
    }
}