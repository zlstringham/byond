use std::num::Wrapping;

/// 4-byte integer hash, full avalanche.
/// https://burtleburtle.net/bob/hash/integer.html
fn hash(a: u32) -> u32 {
    let mut a = (Wrapping(a) + Wrapping(0x7ed55d16)) + (Wrapping(a) << 12);
    a = (a ^ Wrapping(0xc761c23c)) ^ (a >> 19);
    a = (a + Wrapping(0x165667b1)) + (a << 5);
    a = (a + Wrapping(0xd3a2646c)) ^ (a << 9);
    a = (a + Wrapping(0xfd7046c5)) + (a << 3);
    a = (a ^ Wrapping(0xb55a4f09)) ^ (a >> 16);
    a.0
}

#[allow(unused)]
pub(crate) fn encrypt(key: u32, bytes: &mut [u8]) {
    bytes.iter_mut().fold(key, |k, b| {
        let xor = (k & 0xff) as u8;
        *b ^= xor;
        hash((Wrapping(k) + Wrapping(*b as u32)).0)
    });
}

#[allow(unused)]
pub(crate) fn decrypt(key: u32, bytes: &mut [u8]) {
    bytes.iter_mut().fold(key, |mut k, b| {
        let xor = (k & 0xff) as u8;
        k = hash((Wrapping(k) + Wrapping(*b as u32)).0);
        *b ^= xor;
        k
    });
}
