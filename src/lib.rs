// code copied from https://github.com/rust-dataframe/discussion/issues/7
#![feature(const_generics)]

// #[macro_use]
// extern crate frunk;
// extern crate frunk_core;

use frunk::hlist::{HList, HCons, HNil, Selector, Sculptor};
// use frunk::generic::Generic;

#[derive(PartialEq, Eq, Clone, Debug)]
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

// #[derive(Debug, PartialEq)]
// pub struct Row {
//     pub col1: i32,
//     pub col2: f32,
// }

// impl Row {
//     pub fn new() -> Self {
//         Self {
//             col1: 0,
//             col2: 0.0,
//         }
//     }
// }

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

    pub fn get_row<Index, T1: Copy, const S1: &'static str, T2: Copy, const S2: &'static str>(&self, idx: usize) -> (T1, T2)
    where Col: Sculptor<HCons<Column<T1, S1>, HCons<Column<T2, S2>, HNil>>, Index>
    {
        let (target, _): (HCons<Column<T1, S1>, HCons<Column<T2, S2>, HNil>>, _) = self.inner_cols.clone().sculpt();
        let list: (Column<T1, S1>, Column<T2, S2>) = target.into();
        (list.0.inner[idx], list.1.inner[idx])
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
    fn test_dataframe() {
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

        let row = df.get_row::<_, i32, "col1", f32, "col2">(0);
        assert_eq!(1, row.0);
        assert_eq!(1.1, row.1);
    }
}


// impl<T: Copy, const S: &'static str> Generic for Column<T, S> {
//     type Repr = HCons<Column<T, S>, HNil>;
    
//     fn into(self) -> Self::Repr {
//         hlist![self]
//     }

//     fn from(a: Self::Repr) -> Self {
//         a.head
//     }
// }

// impl<T: Copy, const S: &'static str> From<Column<T, S>> for HCons<Column<T, S>, HNil> {
//     fn from(a: Column<T, S>) -> Self {
//         hlist![a]
//     }
// }

// impl Generic for Row {
//     type Repr = HCons<Row, HNil>;
    
//     fn into(self) -> Self::Repr {
//         hlist![self]
//     }

//     fn from(a: Self::Repr) -> Self {
//         a.head
//     }
// }
