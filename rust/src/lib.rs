mod parser;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::catch_unwind;

#[no_mangle]
pub extern "C" fn luna_parse_document(path: *const c_char, chunk_size: usize) -> *mut c_char {
    let res = catch_unwind(|| {
        let path_str = unsafe {
            CStr::from_ptr(path)
                .to_str()
                .map_err(|_| "Invalid UTF-8 path")?
        };
        match parser::parse_and_chunk(path_str, chunk_size) {
            Ok(chunks) => {
                let output = serde_json::json!({
                    "status": "ok",
                    "chunks": chunks,
                    "count": chunks.len()
                });
                serde_json::to_string(&output).map_err(|e| e.to_string())
            }
            Err(e) => Err(e),
        }
    });

    let result_str = match res {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => serde_json::json!({"status": "error", "message": e.to_string()}).to_string(),
        Err(_) => serde_json::json!({"status": "error", "message": "RUST_PANIC"}).to_string(),
    };

    CString::new(result_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn luna_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}
