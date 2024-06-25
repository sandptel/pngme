#![allow(unused_variables)]

use crc::{Crc, Algorithm, CRC_16_IBM_SDLC, CRC_32_ISCSI};

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::error::Error;
use thiserror::Error;

use crate::chunk_type;

use super::chunk_type::ChunkType;

pub struct Chunk{
    data: Vec<u8>,
    chunk_type: ChunkType,
}
pub enum ChunkError{
    InvalidChunkType,
    InvalidData,
    InvalidCRC,
}

impl TryFrom<&[u8]> for Chunk{
    type Error = ChunkError;
    
        fn try_from(bytes: &[u8]) -> Result<Self, ChunkError>{
            
        }
    
}

impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self{
        Self{
            data,
            chunk_type,
        }
    }

    fn length(&self)->u32{
        self.data.len() as u32
    }

    fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }

    fn data(&self) -> &[u8]
    {
        &self.data
    }

    pub fn crc(&self)-> u32{
        let crc = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digest = crc.digest();
        digest.update(&self.chunk_type.bytes());
        digest.update(&self.data);
        digest.finalize()
    }

    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::chunk_type::ChunkType;
        use std::str::FromStr;
    
        fn testing_chunk() -> Chunk {
            let data_length: u32 = 42;
            let chunk_type = "RuSt".as_bytes();
            let message_bytes = "This is where your secret message will be!".as_bytes();
            let crc: u32 = 2882656334;
    
            let chunk_data: Vec<u8> = data_length
                .to_be_bytes()
                .iter()
                .chain(chunk_type.iter())
                .chain(message_bytes.iter())
                .chain(crc.to_be_bytes().iter())
                .copied()
                .collect();
            
            Chunk::try_from(chunk_data.as_ref()).unwrap()
        }
    
        #[test]
        fn test_new_chunk() {
            let chunk_type = ChunkType::from_str("RuSt").unwrap();
            let data = "This is where your secret message will be!".as_bytes().to_vec();
            let chunk = Chunk::new(chunk_type, data);
            assert_eq!(chunk.length(), 42);
            assert_eq!(chunk.crc(), 2882656334);
        }
    
        #[test]
        fn test_chunk_length() {
            let chunk = testing_chunk();
            assert_eq!(chunk.length(), 42);
        }
    
        // #[test]
        // fn test_chunk_type() {
        //     let chunk = testing_chunk();
        //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        // }
    
        // #[test]
        // fn test_chunk_string() {
        //     let chunk = testing_chunk();
        //     let chunk_string = chunk.data_as_string().unwrap();
        //     let expected_chunk_string = String::from("This is where your secret message will be!");
        //     assert_eq!(chunk_string, expected_chunk_string);
        // }
    
        // #[test]
        // fn test_chunk_crc() {
        //     let chunk = testing_chunk();
        //     assert_eq!(chunk.crc(), 2882656334);
        // }
    
        // #[test]
        // fn test_valid_chunk_from_bytes() {
        //     let data_length: u32 = 42;
        //     let chunk_type = "RuSt".as_bytes();
        //     let message_bytes = "This is where your secret message will be!".as_bytes();
        //     let crc: u32 = 2882656334;
    
        //     let chunk_data: Vec<u8> = data_length
        //         .to_be_bytes()
        //         .iter()
        //         .chain(chunk_type.iter())
        //         .chain(message_bytes.iter())
        //         .chain(crc.to_be_bytes().iter())
        //         .copied()
        //         .collect();
    
        //     let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
    
        //     let chunk_string = chunk.data_as_string().unwrap();
        //     let expected_chunk_string = String::from("This is where your secret message will be!");
    
        //     assert_eq!(chunk.length(), 42);
        //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        //     assert_eq!(chunk_string, expected_chunk_string);
        //     assert_eq!(chunk.crc(), 2882656334);
        // }
    
        // #[test]
        // fn test_invalid_chunk_from_bytes() {
        //     let data_length: u32 = 42;
        //     let chunk_type = "RuSt".as_bytes();
        //     let message_bytes = "This is where your secret message will be!".as_bytes();
        //     let crc: u32 = 2882656333;
    
        //     let chunk_data: Vec<u8> = data_length
        //         .to_be_bytes()
        //         .iter()
        //         .chain(chunk_type.iter())
        //         .chain(message_bytes.iter())
        //         .chain(crc.to_be_bytes().iter())
        //         .copied()
        //         .collect();
    
        //     let chunk = Chunk::try_from(chunk_data.as_ref());
    
        //     assert!(chunk.is_err());
        // }
    
        // #[test]
        // pub fn test_chunk_trait_impls() {
        //     let data_length: u32 = 42;
        //     let chunk_type = "RuSt".as_bytes();
        //     let message_bytes = "This is where your secret message will be!".as_bytes();
        //     let crc: u32 = 2882656334;
    
        //     let chunk_data: Vec<u8> = data_length
        //         .to_be_bytes()
        //         .iter()
        //         .chain(chunk_type.iter())
        //         .chain(message_bytes.iter())
        //         .chain(crc.to_be_bytes().iter())
        //         .copied()
        //         .collect();
            
        //     let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
            
        //     let _chunk_string = format!("{}", chunk);
        // }
    }

