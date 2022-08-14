use std::io::{BufWriter, Write};

use byond_crc32::Crc32;
use byteorder::{LittleEndian, WriteBytesExt};

use crate::{error::EncodeError, Resource};

#[derive(Debug)]
pub struct Encoder<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Self {
        Self {
            writer: BufWriter::new(w),
        }
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

        self.writer.write_u8(resource.flags)?;
        // TODO: encrypt.
        let mut crc = Crc32::new();
        crc.update(&resource.data);
        self.writer.write_u32::<LittleEndian>(crc.as_u32())?;
        self.writer
            .write_u32::<LittleEndian>(resource.modified_time)?;
        self.writer
            .write_u32::<LittleEndian>(resource.created_time)?;
        self.writer
            .write_u32::<LittleEndian>(resource.data.len() as u32)?;
        self.writer.write_all(resource.name.as_bytes())?;
        self.writer.write_u8(b'\0')?;
        // TODO: encrypt.
        self.writer.write_all(&resource.data)?;
        self.writer.flush()?;
        Ok(())
    }
}
