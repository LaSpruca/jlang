#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i128),
    FloatingPointInteger(f64),
    Boolean(bool),
}
