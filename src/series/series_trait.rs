use crate::datatypes::dtype::{DataType, RustyDataType};

pub trait SeriesTrait {
    fn name(&self) -> &str;
    fn dtype(&self) -> &DataType;
    /// Underlying chunks.
    // fn chunks(&self) -> &Vec;
    // fn chunks(&self) -> &IterItem;
    fn chunks(&self) -> &Vec<Box<dyn RustyDataType>>;
    // fn count(&self) -> usize;
    // /// Rename the Series.
    // fn rename(&mut self, name: &str);
    // /// Get length of series.
    // fn len(&self) -> usize;
    // /// Check if Series is empty.
    // fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }
}
