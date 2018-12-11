#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
    Text(String),
    Int(i64),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    Literal(Literal),
    Table(Vec<(Literal, Value)>),
    List(Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Value::Literal(lit) => match lit {
                Literal::Text(text) => write!(w, "Text {:?}", text),
                Literal::Int(i) => write!(w, "Int {:?}", i),
            },
            value => write!(w, "{:?}", value),
        }
    }
}

pub fn text<T: Into<String>>(t: T) -> Value {
    Value::Literal(Literal::Text(t.into()))
}
