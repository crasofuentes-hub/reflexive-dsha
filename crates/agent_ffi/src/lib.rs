#![deny(warnings)]
#![deny(clippy::all)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::missing_errors_doc)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use agent_core::{heal_to_fixpoint, HealConfig};

/// Enterprise note:
/// - We keep ABI minimal and deterministic.
/// - Caller owns input buffer.
/// - We allocate output string with CString; caller must free via dsha_free().
#[no_mangle]
pub extern "C" fn dsha_heal_config_to_json(input_utf8: *const c_char) -> *mut c_char {
    if input_utf8.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = unsafe { CStr::from_ptr(input_utf8) };
    let input = match cstr.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return std::ptr::null_mut(),
    };

    let (final_state, trace) = match heal_to_fixpoint(input, HealConfig::default()) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    let obj = serde_json::json!({
        "final": final_state,
        "trace": trace
    });

    let s = match serde_json::to_string_pretty(&obj) {
        Ok(x) => x,
        Err(_) => return std::ptr::null_mut(),
    };

    let out = match CString::new(s) {
        Ok(x) => x,
        Err(_) => return std::ptr::null_mut(),
    };
    out.into_raw()
}

#[no_mangle]
pub extern "C" fn dsha_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
