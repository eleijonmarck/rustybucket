use std::marker::PhantomData;

use crate::datatypes::dtype::{DataType, RustyDataType};

use super::{series_trait::SeriesTrait, Series};

pub trait IntoSeries {
    fn is_series() -> bool {
        false
    }
    fn into_series(self) -> Series;
}
pub struct ChunkedArray<T: RustyDataType + ?Sized> {
    // data: IteratorRef,
    // data: IterItem,
    data: Vec<Box<dyn RustyDataType>>,
    name: String,
    phantom: PhantomData<T>,
    dtype: DataType,
}
pub trait NewChunkedArray<T, N> {
    fn from_slice(name: &str, v: &[N]) -> Self;
}

impl<T> NewChunkedArray<T, T> for ChunkedArray<T>
where
    T: RustyDataType + ?Sized + std::clone::Clone + 'static,
{
    fn from_slice(name: &str, v: &[T]) -> Self {
        ChunkedArray {
            name: name.to_string(),
            dtype: v[0].get_dtype(),
            data: v
                .iter()
                .map(|x| Box::new(x.clone()) as Box<dyn RustyDataType>)
                .collect(),
            phantom: PhantomData,
        }
    }
}

impl<T: RustyDataType + 'static + std::clone::Clone> IntoSeries for ChunkedArray<T>
where
    ChunkedArray<T>: SeriesTrait,
{
    fn into_series(self) -> Series
    where
        Self: Sized,
    {
        Series(Box::new(self))
    }

    fn is_series() -> bool {
        false
    }
}
//
impl<T> SeriesTrait for ChunkedArray<T>
where
    T: RustyDataType + 'static + Clone,
{
    fn dtype(&self) -> &DataType {
        &self.dtype
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn chunks(&self) -> &Vec<Box<dyn RustyDataType>> {
        &self.data
    }
}
pub trait NamedFrom<T, Phantom: ?Sized> {
    /// Initialize by name and values.
    fn new(name: &str, _: T) -> Self;
}

macro_rules! impl_new_series_from_slice {
    ($t:ty) => {
        impl<T: AsRef<$t>> NamedFrom<T, $t> for Series {
            fn new(name: &str, v: T) -> Self {
                ChunkedArray::<$t>::from_slice(name, v.as_ref()).into_series()
            }
        }
    };
}
impl_new_series_from_slice!([i32]);
// impl_new_series_from_slice!(Vec<i32>);
