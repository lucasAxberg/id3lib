use std::fmt;

pub enum StringParseError {
    InvalidByte(u8, usize),
    InvalidWord(u16, usize),
    InvalidLength(usize),
    MissingBOM,
}

#[derive(Clone, Debug)]
/// Errors for the sync-safe integer data type
pub enum SyncSafeError {
    /// The array of bytes used in convertion is the wrong length
    IncorrectLength(usize)
}

impl fmt::Display for SyncSafeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IncorrectLength(length) => write!(f, "expected '4' given: '{}'", length)
        }
    }
}

#[derive(Clone, Debug)]
/// Errors for the ID3 data structure creation functions
pub enum ID3Error {
    /// Did not find a valid header in the 10 bytes read
    HeaderNotFound,
    /// Was not able to read the amount of bytes needed
    NotEnoughBytes,
}

impl fmt::Display for ID3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HeaderNotFound => write!(f, "Header not found in the given bytes"),
            Self::NotEnoughBytes => write!(f, "Not enough bytes to parse in reader")
        }
    }
}
