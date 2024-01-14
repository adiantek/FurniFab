mod deserializer;
mod error;
mod serializer;

use error::Error;

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct UnitStruct;

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct NewType(pub i32);

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct TupleStruct(pub i32, pub i32, pub i32);

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub enum Enum {
        Unit,
        NewType(i32),
        Tuple(i32, i32, i32),
        Struct { a: i32, b: i32, c: i32 },
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct Struct {
        pub a: i32,
        pub b: i32,
        pub c: i32,
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct AdvancedStruct {
        pub a: i32,
        pub b: String,
        pub c: Vec<Struct>,
        pub d: BTreeMap<u64, Enum>,
        pub e: (u64, u8, Struct),
    }

    pub fn new_advanced_struct() -> AdvancedStruct {
        AdvancedStruct {
            a: 1,
            b: "2".to_string(),
            c: vec![Struct { a: 3, b: 4, c: 5 }, Struct { a: 6, b: 7, c: 8 }],
            d: BTreeMap::from([(9, Enum::Unit), (10, Enum::Tuple(11, 12, 13))]),
            e: (
                14,
                15,
                Struct {
                    a: 16,
                    b: 17,
                    c: 18,
                },
            ),
        }
    }
}
