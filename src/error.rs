//! Error definition of gt_reader

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MismatchedLength { expect: usize },
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
