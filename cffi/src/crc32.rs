use core::slice;

use byond_crc32::Crc32;
use libc::size_t;

/// Represents a CRC-32/BYOND checksum computation.
#[repr(C)]
pub struct ByondCrc32 {
    _private: [u8; 0],
}

/// Creates and initializes a new CRC-32/BYOND checksum.
///
/// The pointer returned is heap-allocated and must be freed with `crc32_free`.
#[no_mangle]
pub extern "C" fn crc32_new() -> *mut ByondCrc32 {
    Box::into_raw(Box::new(Crc32::new())) as *mut ByondCrc32
}

/// Creates and initializes an in-progress CRC-32/BYOND checksum.
///
/// This initializer accepts a starting checksum and the length of data used to calculate the
/// checksum up to this point. The length is used in the calculation for `crc32_combine`.
///
/// The pointer returned is heap-allocated and must be freed with `crc32_free`.
#[no_mangle]
pub extern "C" fn crc32_new_with_initial(crc: u32, len: size_t) -> *mut ByondCrc32 {
    Box::into_raw(Box::new(Crc32::new_with_initial(crc, len as u64))) as *mut ByondCrc32
}

/// Creates a CRC-32/BYOND checksum as the combination of two component checksums.
///
/// The pointer returned is heap-allocated and must be freed with `crc32_free`. The two
/// component checksums are not freed by this function and must still be freed with `crc32_free`.
#[no_mangle]
pub unsafe extern "C" fn crc32_combine(
    crc_a_ptr: *const ByondCrc32,
    crc_b_ptr: *const ByondCrc32,
) -> *mut ByondCrc32 {
    Box::into_raw(Box::new(Crc32::combine(
        &*(crc_a_ptr as *const Crc32),
        &*(crc_b_ptr as *const Crc32),
    ))) as *mut ByondCrc32
}

/// Free the heap memory from a CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_free(crc_ptr: *mut ByondCrc32) {
    Box::from_raw(crc_ptr as *mut Crc32);
}

/// Gets the 32-bit value of a CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_as_u32(crc_ptr: *const ByondCrc32) -> u32 {
    (*(crc_ptr as *const Crc32)).as_u32()
}

/// Returns whether or not the CRC-32/BYOND checksum has ingested no data.
#[no_mangle]
pub unsafe extern "C" fn crc32_is_empty(crc_ptr: *const ByondCrc32) -> bool {
    (*(crc_ptr as *const Crc32)).is_empty()
}

/// Returns the length of data ingested by the CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_len(crc_ptr: *const ByondCrc32) -> u64 {
    (*(crc_ptr as *const Crc32)).len()
}

/// Resets a CRC-32/BYOND checksum to the default state.
///
/// This can be useful for reusing a single checksum to hash multiple
/// data streams, rather than allocating separate checksums for each stream.
#[no_mangle]
pub unsafe extern "C" fn crc32_reset(crc_ptr: *mut ByondCrc32) {
    (*(crc_ptr as *mut Crc32)).reset();
}

/// Updates a CRC-32/BYOND checksum with some data.
#[no_mangle]
pub unsafe extern "C" fn crc32_update(crc_ptr: *mut ByondCrc32, data: *const u8, len: size_t) {
    (*(crc_ptr as *mut Crc32)).update(slice::from_raw_parts(data, len))
}
