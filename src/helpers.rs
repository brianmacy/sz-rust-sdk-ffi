//! Marshaling helpers between Rust strings and the Senzing C ABI.
//!
//! The engine allocates response buffers; ownership transfers to the caller, who frees via
//! `SzHelper_free` (NOT libc `free` — they may use different allocators / CRT heaps on Windows).
//!
//! These are intentionally raw (no error-type wrapping) so each consumer can layer its own error
//! conversion on top without this crate depending on either consumer's error type.

use crate::bindings::SzHelper_free;
use std::ffi::{CStr, CString, NulError, c_char, c_void};

/// Borrow a `&str` as a C string for an FFI call. Errors only on an interior NUL.
pub fn str_to_c_string(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}

/// Free an engine-allocated C string.
///
/// # Safety
/// `ptr` must be null or a pointer returned by a Senzing `_helper`/engine function.
#[inline]
pub unsafe fn sz_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { SzHelper_free(ptr as *mut c_void) };
    }
}

/// Convert an engine-allocated C string to a Rust `String` WITHOUT freeing it. Non-UTF-8 bytes are
/// hex-encoded (matching the SDKs' historical behavior).
///
/// # Safety
/// `ptr` must be null or a valid null-terminated C string.
pub unsafe fn c_str_to_string_borrowed(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let cstr = unsafe { CStr::from_ptr(ptr) };
    match cstr.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => hex::encode(cstr.to_bytes()),
    }
}

/// Take ownership of an engine-allocated C string, convert to a Rust `String`, and free it with
/// `SzHelper_free`. Non-UTF-8 bytes are hex-encoded (matching the SDKs' historical behavior).
///
/// # Safety
/// `ptr` must be null or a valid engine-allocated null-terminated C string.
pub unsafe fn c_str_to_string(ptr: *mut c_char) -> String {
    let s = unsafe { c_str_to_string_borrowed(ptr) };
    unsafe { sz_free(ptr) };
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests deliberately exercise only the non-freeing paths (`str_to_c_string`,
    // `c_str_to_string_borrowed`) — this crate has no build.rs linking a real libSz, so a call
    // reaching `sz_free`/`SzHelper_free` would fail to link the test binary. `sz_free`/
    // `c_str_to_string` are exercised by consumers that DO link libSz (senzing-sys, sz-rust-sdk).

    #[test]
    fn str_to_c_string_round_trips_ascii() {
        let cstring = str_to_c_string("hello").unwrap();
        assert_eq!(cstring.to_str().unwrap(), "hello");
    }

    #[test]
    fn str_to_c_string_rejects_interior_nul() {
        assert!(str_to_c_string("bad\0string").is_err());
    }

    #[test]
    fn c_str_to_string_borrowed_null_is_empty() {
        let s = unsafe { c_str_to_string_borrowed(std::ptr::null_mut()) };
        assert_eq!(s, "");
    }

    #[test]
    fn c_str_to_string_borrowed_valid_utf8() {
        let owned = CString::new("hello world").unwrap();
        let s = unsafe { c_str_to_string_borrowed(owned.as_ptr() as *mut c_char) };
        assert_eq!(s, "hello world");
    }

    #[test]
    fn c_str_to_string_borrowed_non_utf8_is_hex_encoded() {
        // 0xFF is never valid UTF-8 on its own.
        let owned = unsafe { CString::from_vec_unchecked(vec![0xFFu8]) };
        let s = unsafe { c_str_to_string_borrowed(owned.as_ptr() as *mut c_char) };
        assert_eq!(s, hex::encode([0xFFu8]));
    }
}
