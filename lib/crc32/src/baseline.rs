use crate::{
    tables::{BYTE_TABLE, WORD_TABLE},
    DEFAULT_CRC32,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct State {
    state: u32,
}

impl State {
    pub fn new(state: u32) -> Self {
        State { state }
    }

    pub fn update(&mut self, buf: &[u8]) {
        self.state = slice_by_16(self.state, buf);
    }

    pub fn as_u32(self) -> u32 {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = DEFAULT_CRC32;
    }
}

#[inline(always)]
pub(crate) fn slice_by_16(mut crc: u32, bytes: &[u8]) -> u32 {
    crc = u32::swap_bytes(crc);
    let chunks = bytes.chunks_exact(16);
    let remainder = chunks.remainder();
    crc = chunks.fold(crc, |mut crc, word| {
        crc ^= u32::from_le_bytes(word[0..4].try_into().unwrap());
        WORD_TABLE[15][(crc & 0xff) as usize]
            ^ WORD_TABLE[14][((crc >> 8) & 0xff) as usize]
            ^ WORD_TABLE[13][((crc >> 16) & 0xff) as usize]
            ^ WORD_TABLE[12][((crc >> 24) & 0xff) as usize]
            ^ WORD_TABLE[11][word[4] as usize]
            ^ WORD_TABLE[10][word[5] as usize]
            ^ WORD_TABLE[9][word[6] as usize]
            ^ WORD_TABLE[8][word[7] as usize]
            ^ WORD_TABLE[7][word[8] as usize]
            ^ WORD_TABLE[6][word[9] as usize]
            ^ WORD_TABLE[5][word[10] as usize]
            ^ WORD_TABLE[4][word[11] as usize]
            ^ WORD_TABLE[3][word[12] as usize]
            ^ WORD_TABLE[2][word[13] as usize]
            ^ WORD_TABLE[1][word[14] as usize]
            ^ WORD_TABLE[0][word[15] as usize]
    });
    crc = u32::swap_bytes(crc);
    slice_by_1(crc, remainder)
}

#[inline(always)]
pub(crate) fn slice_by_1(crc: u32, bytes: &[u8]) -> u32 {
    bytes.iter().fold(crc, |crc, &byte| {
        (crc << 8) ^ BYTE_TABLE[(crc >> 24) as usize ^ byte as usize]
    })
}

#[cfg(test)]
mod tests {
    use crate::golden;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn slice_by_16_matches_golden(crc: u32, bytes: Vec<u8>) -> bool {
        super::slice_by_16(crc, &bytes) == golden(crc, &bytes)
    }

    #[quickcheck]
    fn slice_by_1_matches_golden(crc: u32, bytes: Vec<u8>) -> bool {
        super::slice_by_1(crc, &bytes) == golden(crc, &bytes)
    }
}
