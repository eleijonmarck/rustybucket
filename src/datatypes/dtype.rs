use core::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    I32,
    String,
}
pub type IterItem = dyn Iterator<Item = DataType>;

pub type IteratorRef = Box<IterItem>;
