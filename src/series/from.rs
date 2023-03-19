use std::fmt::{self, Display};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use dyn_clone::DynClone;

use crate::datatypes::dtype::{DataType, IteratorRef};

use super::{series_trait::SeriesTrait, Series};

pub trait ThisDataType: Send + Sync {
    fn get_dtype(self) -> DataType;
}
impl ThisDataType for i32
where
    i32: Sized,
{
    fn get_dtype(self) -> DataType {
        DataType::I32
    }
}
pub trait IntoSeries {
    fn is_series() -> bool {
        false
    }
    fn into_series(self) -> Series;
}
pub struct ChunkedArray<T>
where
    T: ThisDataType,
{
    data: IteratorRef,
    name: String,
    phantom: PhantomData<T>,
    dtype: DataType,
}
pub trait NewChunkedArray<T> {
    fn from_slice(name: &str, v: &[T]) -> Self;
}

impl<T> NewChunkedArray<T> for ChunkedArray<T>
where
    T: ThisDataType + std::clone::Clone + 'static,
{
    fn from_slice(name: &str, v: &[T]) -> Self {
        let arr = v.to_vec();
        let iter: IteratorRef = Box::new(arr.into_iter().map(|v| v.get_dtype()));
        ChunkedArray {
            name: name.to_string(),
            dtype: T::get_dtype(v[0].clone()),
            data: iter,
            phantom: PhantomData,
        }
    }
}

impl<T: ThisDataType + 'static> IntoSeries for ChunkedArray<T>
where
    ChunkedArray<T>: SeriesTrait,
{
    fn into_series(self) -> Series
    where
        Self: Sized,
    {
        Series(Box::new(self))
    }
}
//
impl<T> SeriesTrait for ChunkedArray<T>
where
    T: ThisDataType + 'static + Clone,
{
    fn dtype(&self) -> &DataType {
        &self.dtype
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn chunks(&self) -> &IteratorRef {
        &self.data
    }
}

macro_rules! impl_new_series {
    ($t:ty) => {
        impl Series {
            pub fn new(name: &str, v: &[$t]) -> Self {
                ChunkedArray::<$t>::from_slice(name, v).into_series()
            }
        }
    };
}
impl_new_series!(i32);
