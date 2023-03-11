#![feature(test)]

pub mod series {
    #![allow(dead_code)]

    enum DTypes {
        Categorical(String),
        Standard(String),
    }
    trait SeriesTrait<T> {
        fn count(&self) -> usize;
    }
    pub struct Series<T: Clone> {
        name: String,
        pub data: Vec<T>,
        dtype: String,
    }
    impl<T: Clone> Series<T> {
        pub fn new(name: String, data: Vec<T>, dtype: Option<String>) -> Self {
            let dtype = determine_type::<T>(dtype);
            // TODO: check if dtype is categorical
            // and make optimizations as most values are the same
            Self { name, data, dtype }
        }
        pub fn concat(&self, other: &Self) -> Self
        where
            Self: Sized,
        {
            let mut data = self.data.clone();
            data.extend(other.data.clone());
            Self::new(self.name.clone(), data, Some(self.dtype.clone()))
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

    // series interface
    impl<T: Clone> SeriesTrait<T> for Series<T> {
        fn count(&self) -> usize {
            self.data.len()
        }
    }

    // Implementing slice for custom type
    // You can do that by implementing the Index trait, and bounding the index to SliceIndex:
    // https://stackoverflow.com/a/57203324/3767229
    impl<T: Clone, Idx> std::ops::Index<Idx> for Series<T>
    where
        Idx: std::slice::SliceIndex<[T]>,
    {
        type Output = Idx::Output;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.data[index]
        }
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn slice_a_series_of_size_1000(b: &mut Bencher) {
        fn slice_series(s: &series::Series<i32>) {
            let _ = &s[1..3];
        }
        b.iter(|| {
            let vec = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s = series::Series::new(String::from("hej"), vec, None);
            for _ in 0..1000 {
                test::black_box(slice_series(&s));
            }
        });
    }

    #[bench]
    fn concat_series_together_1000_1000(b: &mut Bencher) {
        b.iter(|| {
            let vec1 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s1 = series::Series::new(String::from("hej"), vec1, None);
            let vec2 = (0..1000).map(|v| v + 1000).collect::<Vec<i32>>();
            let s2 = series::Series::new(String::from("hej"), vec2, None);
            for _ in 0..1000 {
                test::black_box(&s1.concat(&s2));
            }
        });
    }
}
