// code copied from https://github.com/rust-dataframe/discussion/issues/7
#![feature(const_generics)]

use frunk::hlist::{HList, HCons, HNil, Selector, Sculptor};

#[derive(PartialEq, Eq, Clone)]
pub struct Column<T: Copy, const S: &'static str> {
    pub inner: Vec<T>
}

impl<T: Copy, const S: &'static str> Column<T, S> {
    pub fn new(inner: Vec<T>) -> Self {
        Self {
            inner
        }
    }

    pub fn get_data_by_idx(&self, idx: usize) -> T {
        self.inner[idx]
    }
}

pub struct DataFrame<Col: HList + Clone> {
    inner_cols: Col,
    col_names: Vec<&'static str>
}

impl DataFrame<HNil> {
    pub fn new() -> Self {
        Self {
            inner_cols: HNil,
            col_names: Vec::new(),
        }
    }
}

impl<Col: HList + Clone> DataFrame<Col> {
    pub fn add<T: Copy, const S: &'static str>(self, col: Vec<T>) -> DataFrame<HCons<Column<T, S>, Col>> {
        let column = Column::new(col);
        let mut cpy = self.col_names.clone();
        cpy.push(S);
        DataFrame {
            inner_cols: self.inner_cols.prepend(column),
            col_names: cpy,
        }
    }

    pub fn get<Index, T: Copy, const S: &'static str>(&self) -> &Column<T, S>
    where
        Col: Selector<Column<T, S>, Index>,
    {
        Selector::get(&self.inner_cols)
    }

    pub fn get_column<Index, T>(&self) -> T
    where Col: Sculptor<T, Index>
    {
        let (target, _): (T, _) = self.inner_cols.clone().sculpt();
        target
    }

    pub fn schema(&self) -> Vec<&'static str>
    {
        self.col_names.to_owned()
    }

    pub fn get_cell_by_idx<Index, T: Copy, const S: &'static str>(&self, idx: usize) -> T
    where
        Col: Selector<Column<T, S>, Index>,
    {
        let col = self.get::<Index, T, S>();
        col.get_data_by_idx(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_col() {
        let df = DataFrame::new();
        let col1: Vec<i32> = vec![1, 2, 3];
        let col2: Vec<f32> = vec![1.1, 2.1, 3.1];

        let df = df.add::<i32, "col1">(col1);
        let df = df.add::<f32, "col2">(col2);

        assert_eq!(1, df.get_cell_by_idx::<_, i32, "col1">(0));

        let cols1 = df.get::<_, i32, "col1">();
        assert_eq!(1, cols1.inner[0]);

        let cols3 = df.get_column::<_, HCons<Column<f32, "col2">, HNil>>();
        assert_eq!(1.1, cols3.head.inner[0]);

        let cols4 = df.get_column::<_, HCons<Column<i32, "col1">, HNil>>();
        assert_eq!(1, cols4.head.inner[0]);
    }
}
