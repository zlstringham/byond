mod crypt;
mod decode;
mod encode;
mod error;

pub use decode::Decoder;
pub use encode::Encoder;
pub use error::DecodeError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Resource {
    pub flags: u8,
    pub modified_time: u32,
    pub created_time: u32,
    pub name: String,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use crate::{Decoder, Encoder, Resource};

    #[test]
    fn encode_decode() {
        let resource = Resource {
            flags: 4,
            modified_time: 5,
            created_time: 6,
            name: "foo".to_string(),
            data: b"bar".iter().cloned().collect(),
        };
        let mut buf = vec![];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write(&resource).unwrap();
            encoder.write(&resource).unwrap();
        }
        let mut decoder = Decoder::new(buf.as_slice());
        assert_eq!(decoder.read_next().unwrap().unwrap(), resource);
        assert_eq!(decoder.read_next().unwrap().unwrap(), resource);
        assert!(matches!(decoder.read_next(), Ok(None)));
    }

    #[test]
    fn encode_decode_encrypted() {
        let resource = Resource {
            flags: 4,
            modified_time: 5,
            created_time: 6,
            name: "foo".to_string(),
            data: b"bar".iter().cloned().collect(),
        };
        let mut buf = vec![];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.encrypt(true);
            encoder.write(&resource).unwrap();
            encoder.write(&resource).unwrap();
        }
        let mut decoder = Decoder::new(buf.as_slice());
        assert_eq!(decoder.read_next().unwrap().unwrap(), resource);
        assert_eq!(decoder.read_next().unwrap().unwrap(), resource);
        assert!(matches!(decoder.read_next(), Ok(None)));
    }

    #[test]
    fn encode_decode_skipped_resource() {
        let resource = Resource {
            flags: 4,
            modified_time: 5,
            created_time: 6,
            name: "foo".to_string(),
            data: b"bar".iter().cloned().collect(),
        };
        let mut buf = vec![];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write(&resource).unwrap();
            encoder.write(&resource).unwrap();
        }
        buf[4] = 0; // Set the block flag of the first resource to 0.
        let mut decoder = Decoder::new(buf.as_slice());
        assert_eq!(decoder.read_next().unwrap().unwrap(), resource);
        assert!(matches!(decoder.read_next(), Ok(None)));
    }
}
