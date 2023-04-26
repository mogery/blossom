use std::collections::HashMap;

use crate::number::Number;

pub enum ObjectKey {
    String(String),
    Symbol(Option<String>),
}

pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Symbol(Option<String>),
    Number(Number),
    BigInt(i64),
    Object(HashMap<ObjectKey, Value>),
}

impl From<ObjectKey> for Value {
    fn from(value: ObjectKey) -> Self {
        match value {
            ObjectKey::String(x) => Value::String(x),
            ObjectKey::Symbol(x) => Value::Symbol(x),
        }
    }
}
