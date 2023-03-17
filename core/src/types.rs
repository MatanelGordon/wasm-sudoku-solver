pub type StrResult<T> = Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionalValue<T> {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) value: T,
}
