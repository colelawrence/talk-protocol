pub extern crate generational_arena;

mod table {
    use std::collections::btree_map;
    use generational_arena::{Arena, Index};

    struct RowRef(Index);
    struct RowVal(Vec<i64>);

    // struct ValueInt(i64, RowRef);
    struct ColumnInt {
        map: btree_map::BTreeMap<i64, Vec<RowRef>>,
    }

    impl From<Ty> for ColumnInt {
        fn from(_ty: Ty) -> Self {
            ColumnInt { map: btree_map::BTreeMap::new() }
        }
    }

    /// "<Int> claims <Int> is <Int>"
    struct DBTable {
        name: String,
        rows: Arena<RowVal>,
        columns: Vec<ColumnInt>,
    }

    enum Ty { Int }

    impl DBTable {
        fn new(name: String, columns: Vec<Ty>) -> Self {
            DBTable {
                name: name,
                columns: columns.into_iter().map(|ty| ty.into()).collect(),
                rows: Arena::new(),
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
