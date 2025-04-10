use std::fmt;
use std::fmt::Formatter;
use crate::chunk_type::ChunkType;

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: [u8; 4],
    pub message: Vec<u8>,
    pub crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {


        let length = u32::from_be_bytes(value[0..4].try_into().unwrap());

        let chunk_type: [u8; 4] = value[4..8].try_into().unwrap();



        let message = value[8..8 + length as usize].to_vec();

        if message.len() != length as usize {
            return Err("Data length does not match chunk length");
        }

        let crc_start = 8 + length as usize;
        let crc = u32::from_be_bytes(value[crc_start..crc_start + 4].try_into().unwrap());


        Ok(Chunk {
            length,
            chunk_type,
            message,
            crc,
        })
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk {{ length: {}, chunk_type: {}, message: {:?}, crc: {} }}", self.length, ChunkType::try_from(self.chunk_type).unwrap(), self.message, self.crc)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    // fn test_new_chunk() {
    //     let chunk_type = ChunkType::from_str("RuSt").unwrap();
    //     let data = "This is where your secret message will be!".as_bytes().to_vec();
    //     let chunk = Chunk::new(chunk_type, data);
    //     assert_eq!(chunk.length(), 42);
    //     assert_eq!(chunk.crc(), 2882656334);
    // }
    //
    // // // #[test]
    // // fn test_chunk_length() {
    // //     let chunk = testing_chunk();
    // //     assert_eq!(chunk.length(), 42);
    // // }
    // //
    // // #[test]
    // // fn test_chunk_type() {
    // //     let chunk = testing_chunk();
    // //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    // // }
    // //
    // // #[test]
    // // fn test_chunk_string() {
    // //     let chunk = testing_chunk();
    // //     let chunk_string = chunk.data_as_string().unwrap();
    // //     let expected_chunk_string = String::from("This is where your secret message will be!");
    // //     assert_eq!(chunk_string, expected_chunk_string);
    // // }
    //
    // #[test]
    // fn test_chunk_crc() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.crc(), 2882656334);
    // }
    //
    // #[test]
    // fn test_valid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;
    //
    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
    //
    //     let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
    //
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //
    //     assert_eq!(chunk.length(), 42);
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //     assert_eq!(chunk_string, expected_chunk_string);
    //     assert_eq!(chunk.crc(), 2882656334);
    // }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
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

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}