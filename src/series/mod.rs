#![allow(dead_code)]

mod from;
mod series_trait;
use std::fmt::{Debug, Formatter};

pub use crate::datatypes;

use self::series_trait::SeriesTrait;

// # dyn
// the dyn keyword is used to create a trait object
// it means that the type of the object is not known at compile time
// but it is known at runtime
// it is there to say that we use dynamic dispatch and not static dispatch
// https://stackoverflow.com/a/50650071/3767229
// # Box
// Box is a smart pointer that allocates data on the heap
// this is to get the size of the data at runtime
// https://doc.rust-lang.org/std/boxed/struct.Box.html
pub struct Series(Box<dyn SeriesTrait>);

impl Debug for Series {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Series")
            .field("name", &self.0.name())
            .field("dtype", &self.0.dtype())
            .finish()
    }
}

fn determine_type<T>(dtype: Option<String>) -> String {
    let m = match dtype {
        Some(dtype) => {
            if dtype == "categorical" && std::any::type_name::<T>() == "String" {
                String::from("categorical")
            } else {
                String::from("")
            }
        }
        None => String::from(""),
    };
    if m != "" {
        return m;
    }
    let type_name = std::any::type_name::<T>();
    type_name.to_string()
}

#[cfg(test)]
mod tests {
    use crate::series::from::NamedFrom;

    use super::*;
    // use test::Bencher;

    #[test]
    fn new_series() {
        let s2 = Series::new("int series", &[1, 2, 3]);
        println!("{:?}", s2);
    }
    //
    // #[bench]
    // fn slice_a_series_of_size_1000(b: &mut Bencher) {
    //     fn slice_series(s: &Series) {
    //         let _ = &s[1..3];
    //     }
    //     b.iter(|| {
    //         let vec = (0..1000).map(|v| v + 1000).collect().as_slice();
    //         let s = Series::new("hej", vec);
    //         for _ in 0..1000 {
    //             test::black_box(slice_series(&s));
    //         }
    //     });
    // }

    // #[bench]
    // fn concat_series_together_1000_1000(b: &mut Bencher) {
    //     b.iter(|| {
    //         let vec1 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
    //         let s1 = series::Series::new(String::from("hej"), vec1, None);
    //         let vec2 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
    //         let s2 = series::Series::new(String::from("hej"), vec2, None);
    //         for _ in 0..1000 {
    //             test::black_box(&s1.concat(&s2));
    //         }
    //     });
    // }
}
