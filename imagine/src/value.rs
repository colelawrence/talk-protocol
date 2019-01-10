use serde_json::Value as JSONValue;
// use arrayvec::ArrayString;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone)]
pub struct Value(u64, JSONValue);

impl From<JSONValue> for Value {
    fn from(value: JSONValue) -> Self {
        Value(
            // could probably use some tweaking
            calculate_hash(value.to_string()),
            value.clone()
        )
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}

fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.0 == other.0
    }
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(w, "{}", self.1)
    }
}

pub fn text(t: &str) -> Value {
    Value::from(JSONValue::from(t))
}
