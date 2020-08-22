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

pub struct DataFrame<Col: HList> {
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

impl<Col: HList> DataFrame<Col> {
    pub fn add<T: Copy, const S: &'static str>(self, col: Vec<T>) -> DataFrame<HCons<Column<T, S>, Col>> {
        let column = Column::new(col);
        let mut cpy = self.col_names.clone();
        cpy.push(S);
        let a = self.inner_cols.prepend(column);
        DataFrame {
            inner_cols: a,
            col_names: cpy,
        }
    }

    pub fn get<T: Copy, Index, const S: &'static str>(&self) -> &Column<T, S>
    where
        Col: Selector<Column<T, S>, Index>,
    {
        Selector::get(&self.inner_cols)
    }

    pub fn schema(&self) -> Vec<&'static str>
    {
        self.col_names.to_owned()
    }

    pub fn get_cell_by_idx<T: Copy, Index, const S: &'static str>(&self, idx: usize) -> T
    where
        Col: Selector<Column<T, S>, Index>,
    {
        let col = self.get::<T, Index, S>();
        col.get_data_by_idx(idx)
    }

    // pub fn get_row_by_idx<Index, T1: Copy, const S1: &'static str, T2: Copy, const S2: &'static str>(&self, idx: usize) -> Row<T1, S1, T2, S2> 
    // where
    //     H: Selector<Row<T1, S1, T2, S2>, Index>
    // {
    //     // let cell1 = self.get_cell_by_idx::<T1, Index, S1>(idx);
    //     // let cell2 = self.get_cell_by_idx::<T2, Index, S2>(idx);
    //     let c1: T1 = 1;
    //     let c2: T2 = 2;
    //     Row {
    //         inner: (c1, c2)
    //     }
    // }
}

// impl<H, T: HList> HList for HCons<H, T> {
//     const LEN: usize = 1 + <T as HList>::LEN;
//     fn static_len() -> usize {
//         Self::LEN
//     }
// }

// impl<T1: Copy, const S1: &'static str, T2: Copy, const S2: &'static str> Row<T1, S1, T2, S2> {
//     pub fn new(t1: T1, t2: T2) -> Self {
//         Row {
//             inner: (t1, t2)
//         }
//     }
// }

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

        assert_eq!(1, df.get_cell_by_idx::<i32, _, "col1">(0));
    } 
}
