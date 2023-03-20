use core::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    I32,
    String,
}
pub trait RustyDataType {
    fn get_dtype(&self) -> DataType;
}
impl RustyDataType for i32
where
    i32: Sized,
{
    fn get_dtype(&self) -> DataType {
        DataType::I32
    }
}
impl RustyDataType for Vec<i32>
where
    i32: Sized + Clone,
{
    fn get_dtype(&self) -> DataType {
        DataType::I32
    }
}
