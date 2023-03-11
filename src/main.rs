#![allow(dead_code)]

use std::fmt::Debug;

trait SeriesTrait<T: Debug> {
    fn new(name: String, data: Vec<T>) -> Self;
    fn count(&self) -> usize;
}
struct Series<T> {
    name: String,
    data: Vec<T>,
    dtype: String,
}

fn determine_type<T>() -> String {
    let type_name = std::any::type_name::<T>();
    type_name.to_string()
}

// series interface
impl<T: Debug> SeriesTrait<T> for Series<T> {
    fn new(name: String, data: Vec<T>) -> Series<T> 
    {
        Series {
            name,
            data,
            dtype: determine_type::<T>(),
        }
    }
    fn count(&self) -> usize {
        self.data.len()
    }
}

// Implementing slice for custom type
// You can do that by implementing the Index trait, and bounding the index to SliceIndex:
// https://stackoverflow.com/a/57203324/3767229
impl<T,Idx> std::ops::Index<Idx> for Series<T>
where
    Idx: std::slice::SliceIndex<[T]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}

fn main() {
    let s = Series::new(String::from("hej"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("{}",s.count());
    println!("{}",s.dtype);
    println!("the 2 and 3 elements {:?}",&s[1..3]);
    println!("first 10 elements {:?}",&s[..10]);
    
    // string series
    let s = Series::new(String::from("hej2"), vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]);
    println!("{}",s.count());
    println!("{}",s.dtype);
    println!("the 2 and 3 elements {:?}",&s[1..3]);
    println!("first 10 elements {:?}",&s[..10]);
}