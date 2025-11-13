#[derive(Debug, Error)]
pub enum CuckooError {
    #[error("invalid capacity (must be > 0)")]
    InvalidCapacity,

    #[error("filter is full")]
    FilterFull,

    #[error("capacity too large to allocate table")]
    TooLarge,
}
