pub use crate::series::Series;
use std::any;

pub trait NamedFrom<T, Phantom: ?Sized> {
    /// Initialize by name and values.
    fn new(name: &str, _: T) -> Self;
}
// macro_rules! impl_named_from {
//     ($type:ty, $method:ident) => {
//         impl<T: AsRef<$type>> NamedFrom<T, $type> for Series {
//             fn new(name: &str, v: T) -> Self {
//                 array::$method(name, v.as_ref()).into_series()
//             }
//         }
//     };
// }
macro_rules! impl_named_from {
    ($type:ty, $method:ident) => {
        impl<T: AsRef<$type>> NamedFrom<T, $type> for Series {
            fn new(name: &str, v: T) -> Self {
                Series {
                    name: name.to_string(),
                    dtype: any::type_name::<T>().to_string(),
                }
            }
        }
    };
}
/// Creates a (non-null) [`PrimitiveArray`] from a slice of values.
/// # Implementation
/// This is essentially a memcopy and is thus `O(N)`
pub fn from_slice<T: Clone>(slice: &[T]) -> Vec<T> {
    Vec::from(slice)
}

impl_named_from!([i16], from_slice);
impl_named_from!([i32], from_slice);
impl_named_from!([i64], from_slice);
impl_named_from!([f32], from_slice);
impl_named_from!([f64], from_slice);
