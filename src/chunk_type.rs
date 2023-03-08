use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ParseChunkTypeError;

#[derive(Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|x| !x.is_ascii()) {
            Err("Not valid ascii character")
        } else {
            Ok(ChunkType(value[0], value[1], value[2], value[3]))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.0 as char, self.1 as char, self.2 as char, self.3 as char
        )
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sb = s.as_bytes().to_owned();
        if sb.iter().all(|e| e.is_ascii_alphabetic()) {
            Ok(ChunkType(sb[0], sb[1], sb[2], sb[3]))
        } else {
            Err("Not A-Za-z")
        }
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
    fn is_valid(&self) -> bool {
        let all_alphabetic = self.bytes().iter().all(|x| x.is_ascii_alphabetic());
        let valid_reserved_bit = self.is_reserved_bit_valid();
        return all_alphabetic && valid_reserved_bit;
    }

    fn is_critical(&self) -> bool {
        self.bytes()[0].is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.bytes()[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3].is_ascii_lowercase()
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
