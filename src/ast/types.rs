#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i32),
    String(String),
    Dict(Vec<(String, Value)>),
}
