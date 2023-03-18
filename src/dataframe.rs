use std::{collections::BTreeMap, ops::Index, ops::IndexMut};

use crate::series;

#[derive(Debug, Clone)]
struct DataFrame<T: Clone> {
    // BtreeMap is a sorted map
    col_name_to_series: BTreeMap<String, series::Series<T>>,
}

impl<T: Clone> DataFrame<T> {
    pub fn new(series: Vec<series::Series<T>>) -> Self {
        let mut col_name_to_series = BTreeMap::new();
        for s in series {
            col_name_to_series.insert(s.name.clone(), s);
        }
        Self { col_name_to_series }
    }
    pub fn get_column_names(&self) -> Vec<String> {
        self.col_name_to_series.keys().cloned().collect()
    }
}

// You are using Index, which says this in its documentation:
impl<T: Clone> Index<String> for DataFrame<T> {
    type Output = series::Series<T>;
    fn index(&self, key: String) -> &Self::Output {
        &self.col_name_to_series[&key]
    }
}

// If a mutable value is requested, IndexMut is used instead.
impl<T: Clone> IndexMut<String> for DataFrame<T> {
    // need lifetime here because of the return type
    fn index_mut<'a>(&'a mut self, index: String) -> &'a mut series::Series<T> {
        self.col_name_to_series.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_create_df() {
        let s1 = series::Series::new(String::from("a"), vec![1, 2, 3], None);
        let s2 = series::Series::new(String::from("b"), vec![1, 2, 3], None);
        let df = DataFrame::new(vec![s1, s2]);
        println!("{:?}", df.get_column_names());
    }

    #[test]
    fn test_get_columns() {
        let s1 = series::Series::new(String::from("a"), vec![1, 2, 3], None);
        let s2 = series::Series::new(String::from("b"), vec![1, 2, 3], None);
        let df = DataFrame::new(vec![s1, s2]);
        let columns = df.get_column_names();
        assert_eq!(columns, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_subscript_for_df() {
        let s1 = series::Series::new(String::from("a"), vec![1, 2, 3], None);
        let s2 = series::Series::new(String::from("b"), vec![1, 2, 3], None);
        let df = DataFrame::new(vec![s1, s2]);
        let s = &df["a".to_string()];
        println!("{:?}", s);
        let result = panic::catch_unwind(|| {
            let s = &df["c".to_string()];
        });
        if result.is_err() {
            println!("Error: {:?}", "c is not a column name in the dataframe");
        }
    }
}
