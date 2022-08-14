use std::io::{BufWriter, Write};

use byond_crc32::Crc32;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use crate::{crypt::encrypt, error::EncodeError, Resource};

#[derive(Debug)]
pub struct Encoder<W: Write> {
    writer: BufWriter<W>,
    encrypt: bool,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Self {
        Self {
            writer: BufWriter::new(w),
            encrypt: false,
        }
    }

    pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
        self.encrypt = encrypt;
        self
    }

    pub fn write(&mut self, resource: &Resource) -> Result<(), EncodeError> {
        if resource.data.len() > u32::MAX as usize {
            return Err(EncodeError::SizeLimitExceeded(resource.data.len()));
        }

        // flags: u8
        // name terminating '\0': u8
        // crc, created, modified, size: 4x u32
        let mut block_size = 2 * std::mem::size_of::<u8>() + 4 * std::mem::size_of::<u32>();
        block_size += resource.name.as_bytes().len() + resource.data.len();
        self.writer.write_u32::<LittleEndian>(block_size as u32)?;
        self.writer.write_u8(1)?;

        let mut crc = Crc32::new();
        crc.update(&resource.data);
        let encrypt_key;
        if self.encrypt {
            self.writer.write_u8(resource.flags | 0x80)?;
            let mut crc_bytes = [0u8; 4];
            LittleEndian::write_u32(&mut crc_bytes, crc.as_u32());
            encrypt(0x45dd0ba6, &mut crc_bytes);
            encrypt_key = Some(LittleEndian::read_u32(&crc_bytes));
            self.writer.write_all(&crc_bytes)?;
        } else {
            self.writer.write_u8(resource.flags)?;
            self.writer.write_u32::<LittleEndian>(crc.as_u32())?;
            encrypt_key = None;
        }
        self.writer
            .write_u32::<LittleEndian>(resource.modified_time)?;
        self.writer
            .write_u32::<LittleEndian>(resource.created_time)?;
        self.writer
            .write_u32::<LittleEndian>(resource.data.len() as u32)?;
        self.writer.write_all(resource.name.as_bytes())?;
        self.writer.write_u8(b'\0')?;
        if let Some(key) = encrypt_key {
            let mut data = resource.data.clone();
            encrypt(key, &mut data);
            self.writer.write_all(&data)?;
        } else {
            self.writer.write_all(&resource.data)?;
        }
        self.writer.flush()?;
        Ok(())
    }
}
