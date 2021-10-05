use std::os::raw::c_char;
use std::ffi::CStr;
use std::str::Utf8Error;

/// Creates a zeroed Vec with specified `capacity`.
///
/// # Safety
/// There is no guarantee that an all-zero byte-pattern represents a valid value of some type T.
///
/// From [std::mem::zeroed].
pub unsafe fn zeroed_vec<T: Copy>(capacity: usize) -> Vec<T> {
    let mut vec = Vec::<T>::with_capacity(capacity);
    vec.resize(capacity, std::mem::zeroed());

    vec
}

/// Create a String using a string of null-terminated bytes `cstring`.
///
/// # Observation
/// All bytes are copied to the heap.
///
/// # Safety
/// * There is no guarantee to the validity of pointer.
/// * The returned lifetime is not guaranteed to be the actual lifetime of pointer.
/// * There is no guarantee that the memory pointed to by pointer contains a valid nul terminator byte
/// at the end of the string.
///
/// From [std::ffi::CStr::from_ptr].
///
pub unsafe fn from_cstring(cstring: *const c_char) -> Result<String, Utf8Error> {
    let raw = CStr::from_ptr(cstring);

    match raw.to_str() {
        Ok(str) => Ok(String::from(str)),
        Err(e) => Err(e),
    }
}
