//! Maps of maps
mod array;
mod hash_map;

use std::os::unix::io::RawFd;

use crate::maps::MapError;

pub use array::Array;
pub use hash_map::HashMap;

/// Dummy documentation
pub trait MapOfMaps {
    /// Dummy documentation
    fn fd_or_err(&self) -> Result<RawFd, MapError>;
}
