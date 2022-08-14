use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("unexpected block flag {0} (expected 0 or 1)")]
    UnexpectedBlockFlag(u8),
    #[error("size limit exceeded (size {0} exceeds maximum {})", isize::MAX)]
    SizeLimitExceeded(u32),
    #[error("checksum mismatch (expected {expected:#010x}, actual {actual:#010x})")]
    ChecksumMismatch { expected: u32, actual: u32 },
    #[error(transparent)]
    IoError(#[from] io::Error),
}
