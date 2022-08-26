use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::error::Error;


#[derive(Debug)]
pub enum ChunkTypeDecodingError {
    BadByte(u8),
    BadLength(usize),
}
impl fmt::Display for ChunkTypeDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadByte(byte) => write!(f, "Bad byte: {byte} ({byte:b})", byte = byte),
            Self::BadLength(len) => write!(f, "Bad length: {} (expected 4)", len),
        }
    }
}

impl Error for ChunkTypeDecodingError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType{
    ct_bytes: [u8; 4]
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.ct_bytes
    }

    pub fn is_critical(&self) -> bool {
        self.ct_bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.ct_bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.ct_bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.ct_bytes[3].is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in bytes.iter() {
            if !Self::is_valid_byte(*byte){
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }

        Ok(ChunkType { ct_bytes: bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bytes() {
            write!(f, "{}", char::from(*b))?;
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeDecodingError::BadLength(s.len())));
        }

        let mut str_bytes: [u8; 4] = [0; 4];

        for (index, byte) in s.as_bytes().iter().enumerate() {
            if Self::is_valid_byte(*byte){
                str_bytes[index] = *byte;
            }
            else{
                return Err(Box::new(ChunkTypeDecodingError::BadByte(*byte)));
            }
        }

        Ok(ChunkType { ct_bytes: str_bytes })
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

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}