pub trait Numeric {
    fn unary_minus(&self) -> Self;
    fn bitwise_not(&self) -> Self;
    fn exponentiate(&self, y: &Self) -> Self;
    fn multiply(&self, y: &Self) -> Self;
    fn divide(&self, y: &Self) -> Self;
    fn remainder(&self, y: &Self) -> Self;
    fn add(&self, y: &Self) -> Self;
    fn subtract(&self, y: &Self) -> Self;
    fn left_shift(&self, y: &Self) -> Self;
    fn signed_right_shift(&self, y: &Self) -> Self;
    fn unsigned_right_shift(&self, y: &Self) -> Self;
    fn less_than(&self, y: &Self) -> Option<bool>;
    fn equal(&self, y: &Self) -> bool;
    fn same_value(&self, y: &Self) -> bool;
    fn same_value_zero(&self, y: &Self) -> bool;
    fn bitwise_and(&self, y: &Self) -> Self;
    fn bitwise_xor(&self, y: &Self) -> Self;
    fn bitwise_or(&self, y: &Self) -> Self;
    fn to_string(&self) -> String;
}
