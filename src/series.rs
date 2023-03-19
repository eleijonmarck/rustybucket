#![allow(dead_code)]

use serde::Serialize;

#[derive(Serialize)]
pub enum DataType {
    I32,
    String,
}

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
trait SeriesTrait {
    fn count(&self) -> usize;
}
#[derive(Debug, Clone)]
pub struct Series(dyn SeriesTrait);
/// A series is a column in a dataframe
/// It is a vector of values of the same type
/// The type is determined by the dtype field
/// The name is the name of the column
///
/// # Examples
/// let s = Series::new("a", &[1, 2, 3]);
impl Series {
    pub fn new(name: &str, v: T, d_type: &DataType) -> Self {
        let dtype = Self::determine_type(d_type);
        Self {
            name: name.to_string(),
            dtype: dtype.to_string(),
        }
    }
    pub fn concat(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        let mut data = self.data.clone();
        data.extend(other.data.clone());
        Self::new(self.name.clone(), self.dtype.into())
    }
    fn determine_type(dtype: &DataType) -> &str {
        let m = match dtype {
            DataType::I32 => "i32",
            DataType::String => "String",
            _ => "",
        };
        return m;
    }
}

// series interface
impl SeriesTrait for Series {
    fn count(&self) -> usize {
        self.data.len()
    }
}

// Implementing slice for custom type
// You can do that by implementing the Index trait, and bounding the index to SliceIndex:
// https://stackoverflow.com/a/57203324/3767229
impl<Idx> std::ops::Index<Idx> for Series
where
    Idx: std::slice::SliceIndex<u8>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn slice_a_series_of_size_1000(b: &mut Bencher) {
        Series::new("hej", &[1, 2, 3]);
        fn slice_series(s: &Series) {
            let _ = &s[1..3];
        }
        b.iter(|| {
            let vec = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s = Series::new("hej", &[1, 2, 3]);
            for _ in 0..1000 {
                test::black_box(slice_series(&s));
            }
        });
    }

    #[bench]
    fn concat_series_together_1000_1000(b: &mut Bencher) {
        b.iter(|| {
            let vec1 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s1 = Series::new(String::from("hej"), vec1, None);
            let vec2 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s2 = Series::new(String::from("hej"), vec2, None);
            for _ in 0..1000 {
                test::black_box(&s1.concat(&s2));
            }
        });
    }
}
