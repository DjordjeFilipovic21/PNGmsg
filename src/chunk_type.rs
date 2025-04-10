use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Checks if the chunk type is valid.
    /// A valid chunk type must consist of only uppercase and lowercase ASCII letters.
    pub fn is_valid(&self) -> bool {
        self.bytes.iter().all(|&b| b.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    /// Checks if the chunk is critical (first byte is uppercase).
    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    /// Checks if the chunk is public (second byte is uppercase).
    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    /// Checks if the reserved bit is valid (third byte is uppercase).
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    /// Checks if the chunk is safe to copy (fourth byte is lowercase).
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    /// Creates a `ChunkType` from a 4-byte array.
    /// Returns an error if the array contains invalid characters.
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if bytes.iter().all(|&b| b.is_ascii_alphabetic()) {
            Ok(ChunkType {bytes})
        } else {
            Err("ChunkType must contain only ASCII alphabetic characters")
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bytes {
            write!(f, "{}", char::from(*b))?;
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    /// Creates a `ChunkType` from a string.
    /// Returns an error if the string is not exactly 4 characters long or contains invalid characters.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("ChunkType must be exactly 4 characters long");
        }
        let bytes = s.as_bytes();
        if !bytes.iter().all(|&b| b.is_ascii_alphabetic()) {
            return Err("ChunkType must contain only ASCII alphabetic characters");
        }
        Ok(ChunkType {bytes : [bytes[0], bytes[1], bytes[2], bytes[3]]})
    }
}


