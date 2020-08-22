// code copied from https://github.com/rust-dataframe/discussion/issues/7
#![feature(const_generics)]

use frunk::hlist::{HList, HCons, HNil, Selector};

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

pub struct DataFrame<H: HList> {
    inner: H,
    col_names: Vec<&'static str>
}

impl DataFrame<HNil> {
    pub fn new() -> Self {
        Self {
            inner: HNil,
            col_names: Vec::new(),
        }
    }
}

impl<H: HList> DataFrame<H> {
    pub fn add<T: Copy, const S: &'static str>(self, col: Vec<T>) -> DataFrame<HCons<Column<T, S>, H>> {
        let column = Column::new(col);
        let mut cpy = self.col_names.clone();
        cpy.push(S);
        let a = self.inner.prepend(column);
        DataFrame {
            inner: a,
            col_names: cpy,
        }
    }

    pub fn get<T: Copy, Index, const S: &'static str>(&self) -> &Column<T, S>
    where
        H: Selector<Column<T, S>, Index>,
    {
        Selector::get(&self.inner)
    }

    pub fn schema(&self) -> Vec<&'static str>
    {
        self.col_names.to_owned()
    }

    pub fn get_data_by_idx<T: Copy, Index, const S: &'static str>(&self, idx: usize) -> T 
    where
        H: Selector<Column<T, S>, Index>,
    {
        let col = self.get::<T, Index, S>();
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

        assert_eq!(1, df.get_data_by_idx::<i32, _, "col1">(0));
    } 
}
