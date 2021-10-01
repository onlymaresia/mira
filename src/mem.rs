use std::os::raw::c_char;
use std::ffi::CStr;
use std::str::Utf8Error;

pub unsafe fn zeroed_vec<T: Copy>(capacity: usize) -> Vec<T> {
    let mut vec = Vec::<T>::with_capacity(capacity);
    vec.resize(capacity, std::mem::zeroed());

    vec
}

/// Create a `String` using a string of null-terminated bytes, all bytes are copied to the heap.
///
/// # Arguments
///
/// * `cstring` - A string of null-terminated bytes
///
/// # Examples
/// ```rust
/// use std::os::raw::c_char;
/// use std::error::Error;
/// use mira::mem::from_cstring;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let cstring = b"kyaaa!!!\0";
///
///     let string = unsafe { from_cstring(cstring.as_ptr() as *const c_char)? };
///     assert_eq!(string, "kyaaa!!!");
///
///     Ok(())
/// }
/// ```
///
/// # Safety
/// `cstring` can be a null or invalid address.
///
///
pub unsafe fn from_cstring(cstring: *const c_char) -> Result<String, Utf8Error> {
    let raw = CStr::from_ptr(cstring);

    match raw.to_str() {
        Ok(str) => Ok(String::from(str)),
        Err(e) => Err(e),
    }
}
