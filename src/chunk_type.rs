
#![allow(unused_variables)]

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::error::Error;
use thiserror::Error;
// use crate::{Error, Result};

#[derive(Debug,Error, Clone, PartialEq, Eq)]
pub enum ChunkTypeError {
    #[error("Invalid chunk type")]
    InvalidChunkType,
}
#[derive(Debug,Error, Clone, PartialEq, Eq)]
pub struct ChunkType {
    // Write this to store the raw bytes of the chunk type
    bytes: [u8; 4],
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        todo!()
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        todo!()
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        todo!()
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        todo!()
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        for byte in self.bytes().iter()
        {
            if !Self::is_valid_byte(*byte)
            {
                return false;
            }
        }
        return true;
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        byte>=65 && byte<=90 || byte>=97 && byte<=122
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(bytes: [u8; 4]) -> Result<Self,ChunkTypeError> {
        let chunk_type = ChunkType { bytes };
        // todo!()
        if !chunk_type.is_valid()
        {
            return Err(ChunkTypeError::InvalidChunkType);
        }
        Ok(chunk_type)
        
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self,ChunkTypeError> {
        let bytes:Vec<u8>= s.chars().map(|letter|letter as u8).collect();
        ChunkType::try_from(<Vec<u8> as TryInto<[u8;4]>>::try_into(bytes).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }
}

