#[derive(Clone, Debug, PartialEq)]
pub enum IR {
    VStack(Vec<IR>),
    HStack(Vec<IR>),
    Text(String),
    Button(String),
    Image(String), // Added Image variant
    Spacer,
}
