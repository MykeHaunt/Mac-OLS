use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::CStr;
use crate::error::IoError;

#[repr(C)]
pub struct BinaryFile {
    pub data: Vec<u8>,
    pub path: PathBuf,
}

impl BinaryFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let data = fs::read(&path)?;
        if data.is_empty() {
            return Err(IoError::EmptyFile);
        }
        Ok(Self {
            data,
            path: path.as_ref().to_path_buf(),
        })
    }
}
