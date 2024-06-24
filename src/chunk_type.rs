
#![allow(unused_variables)]

use std::arch::x86_64;

use crate::chunk_type;
fn main() {
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

// use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data:Vec<u8>,
}

pub enum Error {
    InvalidByteValue,
    // Other error variants...
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        todo!()
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
        todo!()
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        todo!()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self,Error> {
        let mut data = vec![];
        for &byte in bytes.iter()
        {
            if byte<=255{
             data.push(byte);   
            }
            else {
                return Err(Error::InvalidByteValue);
            }
        }
        let chunk_type = ChunkType{data:data};
        Ok(chunk_type)
}

// impl fmt::Display for ChunkType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// impl FromStr for ChunkType {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self> {
//         todo!()
//     }
// }
}
}


mod tests{
    use super::chunk_type;

#[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }
}