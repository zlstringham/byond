use core::slice;

use byond_crc32::Crc32;
use libc::size_t;

/// Represents a CRC-32/BYOND checksum computation.
#[repr(C)]
pub struct Crc32Struct {
    _private: [u8; 0],
}

/// Creates and initializes a new CRC-32/BYOND checksum.
///
/// The pointer returned must be freed with `crc32_destroy`.
#[no_mangle]
pub extern "C" fn crc32_create() -> *mut Crc32Struct {
    Box::into_raw(Box::new(Crc32::new())) as *mut Crc32Struct
}

/// Creates and initializes an in-progress CRC-32/BYOND checksum.
///
/// This initializer accepts a starting checksum and the length of data used to
/// calculate the checksum up to this point. The length is used in the
/// calculation for `crc32_combine`.
///
/// The pointer returned must also be freed with `crc32_destroy`.
#[no_mangle]
pub extern "C" fn crc32_create_with_initial(crc: u32, len: size_t) -> *mut Crc32Struct {
    Box::into_raw(Box::new(Crc32::new_with_initial(crc, len as u64))) as *mut Crc32Struct
}

/// Creates a CRC-32/BYOND checksum as the combination of two component
/// checksums.
///
/// The pointer returned must be freed with `crc32_destroy`. The two component
/// checksums are not freed by this function and must still be freed with
/// `crc32_destroy`.
#[no_mangle]
pub unsafe extern "C" fn crc32_combine(
    crc_a_ptr: *const Crc32Struct,
    crc_b_ptr: *const Crc32Struct,
) -> *mut Crc32Struct {
    match (
        (crc_a_ptr as *const Crc32).as_ref(),
        (crc_b_ptr as *const Crc32).as_ref(),
    ) {
        (Some(a), Some(b)) => Box::into_raw(Box::new(Crc32::combine(a, b))) as *mut Crc32Struct,
        (_, _) => std::ptr::null_mut(),
    }
}

/// Free the heap memory from a CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_destroy(crc_ptr: *mut Crc32Struct) {
    if !crc_ptr.is_null() {
        let _ = Box::from_raw(crc_ptr as *mut Crc32);
    }
}

/// Gets the 32-bit value of a CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_as_u32(crc_ptr: *const Crc32Struct) -> u32 {
    if let Some(crc) = (crc_ptr as *const Crc32).as_ref() {
        return crc.as_u32();
    }
    0xffffffff
}

/// Returns whether or not the CRC-32/BYOND checksum has ingested no data.
#[no_mangle]
pub unsafe extern "C" fn crc32_is_empty(crc_ptr: *const Crc32Struct) -> bool {
    if let Some(crc) = (crc_ptr as *const Crc32).as_ref() {
        return crc.is_empty();
    }
    true
}

/// Returns the length of data ingested by the CRC-32/BYOND checksum.
#[no_mangle]
pub unsafe extern "C" fn crc32_len(crc_ptr: *const Crc32Struct) -> u64 {
    if let Some(crc) = (crc_ptr as *const Crc32).as_ref() {
        return crc.len();
    }
    0
}

/// Resets a CRC-32/BYOND checksum to the default state.
///
/// This can be useful for reusing a single checksum to hash multiple
/// data streams, rather than allocating separate checksums for each stream.
#[no_mangle]
pub unsafe extern "C" fn crc32_reset(crc_ptr: *mut Crc32Struct) {
    if let Some(crc) = (crc_ptr as *mut Crc32).as_mut() {
        crc.reset();
    }
}

/// Updates a CRC-32/BYOND checksum with some data.
#[no_mangle]
pub unsafe extern "C" fn crc32_update(crc_ptr: *mut Crc32Struct, data: *const u8, len: size_t) {
    match (data.is_null(), (crc_ptr as *mut Crc32).as_mut()) {
        (false, Some(crc)) => crc.update(slice::from_raw_parts(data, len)),
        (_, _) => (),
    }
}
