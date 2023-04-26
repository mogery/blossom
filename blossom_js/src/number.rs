use crate::numeric::Numeric;

pub enum Number {
    Float(f64),
    Int(i64),
}

impl Numeric for Number {
    fn add(&self, y: &Self) -> Self {
        match self {
            Number::Int(x) => {
                
            }
        }
    }
}