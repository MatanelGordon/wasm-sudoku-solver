use std::fmt::{Display, Formatter};

pub type StrResult<T> = Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionalValue<T> {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) value: T,
}

impl<T> Display for PositionalValue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}) -> {}", self.row, self.col, self.value)
    }
}
