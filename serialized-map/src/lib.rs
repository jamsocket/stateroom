use std::collections::BTreeMap;
pub type ByteByteMap = BTreeMap<Vec<u8>, Vec<u8>>;

// File format:
// 0xa8 0x32 (magic number)
// 0x00      (version)
// 0x03      (header entries)
// (header entries)
// 0x08      (main entries)

// 0x0


#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum DeserializeError {
    #[error("Bad magic number")]
    BadMagicNumber,

    #[error("Unexpected end of file")]
    UnexpectedEof,
}

// TODO:
// - [ ] Brotli compression
// - [ ] Header data (version, format version, flavor, date, etc.)
// - [x] Magic number
// - [ ] Delta files
// - [ ] Variable-length encoding of length

pub fn write_length_prefixed(
    writer: &mut impl std::io::Write,
    value: &[u8],
) -> std::io::Result<()> {
    let len = value.len() as u32;
    // write the length as a u32 in big endian
    let mut len_bytes = [0; 4];
    len_bytes.copy_from_slice(&len.to_be_bytes());
    writer.write_all(&len_bytes)?;
    writer.write_all(value)?;
    Ok(())
}

pub fn read_length_prefixed(reader: &mut impl std::io::Read) -> Result<Vec<u8>, DeserializeError> {
    let mut len_bytes = [0; 4];
    reader.read_exact(&mut len_bytes).map_err(|_| DeserializeError::UnexpectedEof)?;
    let len = u32::from_be_bytes(len_bytes);
    let mut value = vec![0; len as usize];
    reader.read_exact(&mut value).map_err(|_| DeserializeError::UnexpectedEof)?;
    Ok(value)
}

pub fn serialize(map: &ByteByteMap, mut writer: impl std::io::Write) -> std::io::Result<()> {
    let magic = [0xa8, 0x32];
    writer.write_all(&magic)?;

    let len = map.len() as u32;
    // write the length as a u32 in big endian
    let mut len_bytes = [0; 4];
    len_bytes.copy_from_slice(&len.to_be_bytes());
    writer.write_all(&len_bytes)?;
    for (key, value) in map {
        write_length_prefixed(&mut writer, key.as_slice())?;
        write_length_prefixed(&mut writer, value.as_slice())?;
    }
    Ok(())
}

pub fn serialize_to_bytes(map: &ByteByteMap) -> Vec<u8> {
    let mut writer = Vec::new();
    serialize(map, &mut writer).unwrap();
    writer
}

pub fn deserialize(mut reader: impl std::io::Read) -> Result<ByteByteMap, DeserializeError> {
    let mut magic_buf = [0; 2];
    reader.read_exact(&mut magic_buf).map_err(|_| DeserializeError::UnexpectedEof)?;

    if magic_buf != [0xa8, 0x32] {
        return Err(DeserializeError::BadMagicNumber)
    }

    let mut map = BTreeMap::new();
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).map_err(|_| DeserializeError::UnexpectedEof)?;
    let len = u32::from_be_bytes(buf);
    for _ in 0..len {
        let key = read_length_prefixed(&mut reader)?;
        let value = read_length_prefixed(&mut reader)?;
        map.insert(key, value);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_map() {
        let map: ByteByteMap = BTreeMap::new();
        let bytes = serialize_to_bytes(&map);
        assert_eq!(deserialize(bytes.as_slice()).unwrap(), map);
    }

    #[test]
    fn single_entry() {
        let map = BTreeMap::from([(b"foo".to_vec(), b"bar".to_vec())]);
        let bytes = serialize_to_bytes(&map);
        assert_eq!(deserialize(bytes.as_slice()).unwrap(), map);
    }

    #[test]
    fn multiple_entries() {
        let map = BTreeMap::from([
            (b"foo".to_vec(), b"bar".to_vec()),
            (b"baz".to_vec(), b"qux".to_vec()),
        ]);

        let bytes = serialize_to_bytes(&map);
        assert_eq!(deserialize(bytes.as_slice()).unwrap(), map);
    }

    #[test]
    fn bad_magic_number() {
        let c: Vec<u8> = b"1234".to_vec();
        let d: &[u8] = &c;

        let result = deserialize(d);

        assert_eq!(result.unwrap_err(), DeserializeError::BadMagicNumber);
    }
}
