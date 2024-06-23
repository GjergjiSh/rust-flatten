use a2l_items::Characteristic;

pub trait Flatten {
    fn a2l_flatten(&self) -> Option<Vec<Characteristic>> {
        None
    }
}

impl Flatten for u32 {

}

impl Flatten for String {

}

impl Flatten for (i32, String) {

}