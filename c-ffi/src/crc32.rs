use core::slice;

use byond_crc32::Crc32;
use libc::size_t;

pub const DEFAULT_CRC32: u32 = 0xffffffff;

/// Creates a CRC-32/BYOND checksum as the combination of two component
/// checksums.
#[no_mangle]
pub extern "C" fn crc32_combine(crc_a: u32, crc_b: u32, crc_b_len: u64) -> u32 {
    Crc32::combine(
        &Crc32::new_with_initial(crc_a, 0),
        &Crc32::new_with_initial(crc_b, crc_b_len),
    )
    .as_u32()
}

/// Updates a CRC-32/BYOND checksum with some data.
///
/// # Safety
///
/// The given len should not exceed the length of data.
#[no_mangle]
pub unsafe extern "C" fn crc32_update(crc: u32, data: *const u8, len: size_t) -> u32 {
    if data.is_null() {
        crc
    } else {
        let mut crc32 = Crc32::new_with_initial(crc, 0);
        crc32.update(slice::from_raw_parts(data, len));
        crc32.as_u32()
    }
}
