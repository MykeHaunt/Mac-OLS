pub mod binary;
pub mod mapdetector;
pub mod error;

#[no_mangle]
pub extern "C" fn ecutil_load_file(path: *const c_char) -> *mut BinaryFile {
    // C-ABI implementation
}
