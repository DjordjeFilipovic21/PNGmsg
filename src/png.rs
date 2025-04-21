use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::{BufReader, Read};

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    const STANDARD_HEADER: [u8; 8] = *b"\x89PNG\r\n\x1a\n";

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Png { chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }


    pub fn remove_chunk(&mut self, chunk_type: ChunkType) -> Result<Chunk, ChunkNotFoundError> {
        if let Some(pos) = self
            .chunks
            .iter()
            .position(|c| c.chunk_type() == &chunk_type)
        {
            Ok(self.chunks.remove(pos))
        } else {
            Err(ChunkNotFoundError {
                chunk_type: chunk_type.to_string(),
            })
        }
    }

    #[allow(dead_code)]
    fn header(&self) -> &[u8; 8] {
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &[Chunk] {
        self.chunks.as_slice()
    }

    pub fn chunk_by_type(&self, chunk_type: ChunkType) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.chunk_type() == &chunk_type)
    }

    /// [Chunk](../chunk/struct.Chunk.html).
    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_iterators: Vec<u8> = self.chunks.iter().map(|c| c.as_bytes()).flatten().collect();
        self.header()
            .iter()
            .chain(chunk_iterators.iter())
            .copied()
            .collect()
    }
}
impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in &self.chunks {
            write!(f, "{}", chunk)?;
        }
        Ok(())
    }
}
impl TryFrom<&[u8]> for Png {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(bytes);
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;
        if header != Png::STANDARD_HEADER {
            return Err(PngDecodeError::boxed(format!(
                "Bad header (received {:?}, expected {:?})",
                header,
                Png::STANDARD_HEADER
            )));
        }

        let mut chunks = Vec::new();
        let mut length_buffer = [0u8; 4];
        while let Ok(()) = reader.read_exact(&mut length_buffer) {
            let final_position = 4 + u32::from_be_bytes(length_buffer) + 4;
            let mut buffer = vec![0; usize::try_from(final_position)?];
            reader.read_exact(&mut buffer)?;
            let all_bytes: Vec<u8> = length_buffer
                .iter()
                .copied()
                .chain(buffer.into_iter())
                .collect();
            let chunk = Chunk::try_from(all_bytes.as_slice())?;
            chunks.push(chunk);
        }
        Ok(Png::from_chunks(chunks))
    }
}

#[derive(Debug)]
pub struct PngDecodeError {
    reason: String,
}
impl PngDecodeError {
    fn boxed(reason: String) -> Box<Self> {
        Box::new(Self { reason })
    }
}

impl fmt::Display for PngDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad PNG: {}", self.reason)
    }
}
impl Error for PngDecodeError {}

#[derive(Debug)]
pub struct ChunkNotFoundError {
    chunk_type: String,
}
impl Error for ChunkNotFoundError {}
impl fmt::Display for ChunkNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk not found with type {}", self.chunk_type)
    }
}


