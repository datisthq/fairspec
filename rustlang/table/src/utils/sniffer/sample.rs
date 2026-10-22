#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleSizeType {
    Records,
    Bytes,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleSize {
    pub r#type: SampleSizeType,
    pub count: usize,
}
