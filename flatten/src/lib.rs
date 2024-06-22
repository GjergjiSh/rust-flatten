use a2l_items::Characteristic;

pub trait Flatten {
    fn a2l_flatten(&self) -> Vec<Characteristic>;
}