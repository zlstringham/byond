//! ## Example
//!
//! ```rust
//! use byond_crc32::Crc32;
//!
//! let mut crc32 = Crc32::new();
//! crc32.update(b"123456789");
//! let checksum = crc32.as_u32();
//! ```

#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod baseline;
mod combine;
pub mod specialized;
mod tables;

#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

const DEFAULT_CRC32: u32 = 0xffffffff;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Baseline(baseline::State),
    Specialized(specialized::State),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Represents an in-progress CRC-32/BYOND computation.
pub struct Crc32 {
    len: u64,
    state: State,
}

impl Crc32 {
    /// Creates a new CRC-32/BYOND computation hasher.
    pub fn new() -> Self {
        Self::new_with_initial(DEFAULT_CRC32, 0)
    }

    /// Creates a new CRC-32/BYOND computation hasher with the
    /// given initial checksum.
    ///
    /// The `len` parameter represents the amount of bytes consumed to
    /// create the existing checksum, and is used when combining checksums.
    pub fn new_with_initial(crc: u32, len: u64) -> Self {
        let state = specialized::State::new(crc).map_or_else(
            || State::Baseline(baseline::State::new(crc)),
            State::Specialized,
        );
        Self { len, state }
    }

    /// Gets the underlying checksum value.
    pub fn as_u32(&self) -> u32 {
        match self.state {
            State::Baseline(ref state) => state.as_u32(),
            State::Specialized(ref state) => state.as_u32(),
        }
    }

    /// Returns true if no data has been consumed so far.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The length of data consumed to create the current checksum value.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Resets the CRC-32/BYOND computation hasher to its initial state.
    pub fn reset(&mut self) {
        self.len = 0;
        match self.state {
            State::Baseline(ref mut state) => state.reset(),
            State::Specialized(ref mut state) => state.reset(),
        }
    }

    /// Updates the CRC-32/BYOND computation with the given `bytes`.
    pub fn update(&mut self, bytes: &[u8]) {
        self.len += bytes.len() as u64;
        match self.state {
            State::Baseline(ref mut state) => state.update(bytes),
            State::Specialized(ref mut state) => state.update(bytes),
        }
    }

    /// Combines two CRC-32/BYOND checksums.
    pub fn combine(a: &Self, b: &Self) -> Self {
        let (crc1, crc2) = (a.as_u32(), b.as_u32());
        Self::new_with_initial(combine::combine(crc1, crc2, b.len), a.len + b.len)
    }
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Crc32 {
    fn finish(&self) -> u64 {
        self.as_u32() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl PartialEq<u32> for Crc32 {
    fn eq(&self, &other: &u32) -> bool {
        self.as_u32() == other
    }
}

impl PartialEq<Crc32> for u32 {
    fn eq(&self, other: &Crc32) -> bool {
        *self == other.as_u32()
    }
}

#[cfg(test)]
fn golden(crc: u32, bytes: &[u8]) -> u32 {
    bytes.iter().fold(crc, |mut crc, &byte| {
        crc ^= u32::from(byte) << 24;
        for _ in 0..8 {
            crc = if crc & 0x80000000 != 0 {
                (crc << 1) ^ 0xaf
            } else {
                crc << 1
            };
        }
        crc
    })
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::golden;

    const CHECK: u32 = 0xa5fd3138;

    #[test]
    fn golden_is_valid() {
        assert_eq!(CHECK, golden(crate::DEFAULT_CRC32, b"123456789"));
    }

    #[quickcheck]
    fn check(data: Vec<u8>) -> bool {
        let mut crc32 = super::Crc32::new();
        crc32.update(data.as_slice());
        crc32.as_u32() == golden(crate::DEFAULT_CRC32, data.as_slice())
    }

    #[test]
    fn check_combine() {
        let mut crc_a = super::Crc32::new();
        let mut crc_b = super::Crc32::new();
        crc_a.update(b"12345");
        crc_b.update(b"6789");
        assert_eq!(CHECK, super::Crc32::combine(&crc_a, &crc_b));
    }
}
