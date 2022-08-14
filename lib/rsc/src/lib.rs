mod crypt;
mod decode;
mod encode;
mod error;

pub use decode::Decoder;
pub use encode::Encoder;
pub use error::DecodeError;

#[derive(Clone, Debug, Default)]
pub struct Resource {
    pub flags: u8,
    pub modified_time: u32,
    pub created_time: u32,
    pub name: String,
    pub data: Vec<u8>,
}
