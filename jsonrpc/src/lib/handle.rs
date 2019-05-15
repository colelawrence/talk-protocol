use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::{Serialize, Deserialize};
use erased_serde::Serialize;
// macros
use erased_serde::{serialize_trait_object, __internal_serialize_trait_object};

pub trait SerializeDebug: erased_serde::Serialize + std::fmt::Debug {}
serialize_trait_object!(SerializeDebug);

#[derive(Serialize, Deserialize, Debug)]
pub struct RTEntity {
    id: String,
    note: String,
}

#[derive(Serialize, Debug)]
pub struct NewQuery {
    pub query: Vec<String>,
    pub defs: Option<HashMap<String, Box<SerializeDebug>>>,
}

#[derive(Serialize, Debug)]
pub struct NewStatement {
    template: String,
    values: Vec<Box<SerializeDebug>>,
}

impl NewStatement {
    pub fn new(template: String, values: Vec<Box<SerializeDebug>>) -> Self {
        NewStatement { template, values }
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryMatch {
    values: HashMap<String, String>,
}

impl QueryMatch {
    pub fn get_value<'a, T>(&'a self, name: &str) -> Option<T>
    where T: Deserialize<'a> {
        self.values.get(name)
            .map(|val| serde_json::from_str::<T>(val).expect("no parsing errors"))
    }
}

pub trait RTHandle
where Self: std::marker::Sized {
    fn new_entity(note: String) -> RTEntity;
    fn listen(query: NewQuery, callback: impl Fn(Self, Option<QueryMatch>));
    fn insert(statement: NewStatement);
}
