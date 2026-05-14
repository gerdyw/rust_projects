#[derive(PartialEq, Debug)]
pub enum Cond {
    Negative,
    Zero,
    Positive
}

impl Default for Cond {
    fn default() -> Self {
        Cond::Zero
    }
}