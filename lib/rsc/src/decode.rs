use std::io::{self, BufRead, BufReader, ErrorKind, Read};

use byond_crc32::Crc32;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

use crate::{crypt::decrypt, error::DecodeError, Resource};

pub struct Decoder<R: Read> {
    reader: BufReader<R>,
    skip_checksum: bool,
}

impl<R: Read> Decoder<R> {
    pub fn new(r: R) -> Self {
        Self {
            reader: BufReader::new(r),
            skip_checksum: false,
        }
    }

    pub fn skip_checksum(&mut self, skip_checksum: bool) -> &mut Self {
        self.skip_checksum = skip_checksum;
        self
    }

    pub fn read_next(&mut self) -> Result<Option<Resource>, DecodeError> {
        loop {
            let mut block_info = [0u8; 5];
            match self.reader.read(&mut block_info) {
                Ok(0) => return Ok(None),
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
                _ => (),
            };

            match block_info[4] {
                0 => {
                    // Skip this block.
                    let block_size = LittleEndian::read_u32(&block_info[0..4]);
                    io::copy(
                        &mut self.reader.by_ref().take(block_size.into()),
                        &mut io::sink(),
                    )?;
                    continue;
                }
                1 => break,
                n => return Err(DecodeError::UnexpectedBlockFlag(n)),
            };
        }

        let mut flags = self.reader.read_u8()?;
        let mut crc_bytes = [0u8; 4];
        self.reader.read_exact(&mut crc_bytes)?;
        let modified_time = self.reader.read_u32::<LittleEndian>()?;
        let created_time = self.reader.read_u32::<LittleEndian>()?;
        let size = self.reader.read_u32::<LittleEndian>()?;
        // Ensure we don't panic due to allocating beyond Vec capacity limits.
        if size > isize::MAX as u32 {
            return Err(DecodeError::SizeLimitExceeded(size));
        }
        let mut name_bytes = vec![];
        self.reader.read_until(b'\0', &mut name_bytes)?;
        if name_bytes[name_bytes.len() - 1] == b'\0' {
            name_bytes.pop();
        }
        let mut data = vec![0; size as usize];
        self.reader.read_exact(&mut data)?;

        let crc;
        if flags & 0x80 != 0 {
            decrypt(0x45dd0ba6, &mut crc_bytes);
            crc = LittleEndian::read_u32(&crc_bytes);
            decrypt(crc, &mut data);
            flags &= 0x7f;
        } else {
            crc = LittleEndian::read_u32(&crc_bytes);
        }

        if !self.skip_checksum {
            let mut checksum = Crc32::new();
            checksum.update(&data);
            if checksum != crc {
                return Err(DecodeError::ChecksumMismatch {
                    expected: crc,
                    actual: checksum.as_u32(),
                });
            }
        }

        Ok(Some(Resource {
            flags,
            modified_time,
            created_time,
            data,
            name: String::from_utf8_lossy(&name_bytes).into_owned(),
        }))
    }

    pub fn read_all(&mut self) -> Result<Vec<Resource>, DecodeError> {
        let mut result = vec![];
        while let Some(r) = self.read_next()? {
            result.push(r);
        }
        Ok(result)
    }
}
