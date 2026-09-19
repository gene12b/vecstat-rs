use thiserror::Error;

#[derive(Debug, Error)]
pub enum VecStatError<T> {
    #[error(
        "Vecs have unequal lengths: vec 'A' len {0}, vec 'B' {1}, vec 'A': {2:?}, vec 'B': {3:?}"
    )]
    UnequalLengths(usize, usize, Vec<T>, Vec<T>),
    #[cfg(feature = "nonempty")]
    #[error(
        "Vecs have unequal lengths: vec 'A' len {0}, vec 'B' {1}, vec 'A': {2:?}, vec 'B': {3:?}"
    )]
    UnequalLengthsNonEmpty(usize, usize, nonempty::NonEmpty<T>, nonempty::NonEmpty<T>),
}
