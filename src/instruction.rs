#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    PushInt(i64),
    Add,
    Sub,
    Mul,
    Div,
}