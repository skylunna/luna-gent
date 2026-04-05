mod index;
mod parser;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::panic;

/// FFI 安全包装器：捕获 Panic，自动将 Result<String, String> 转为 *mut c_char
macro_rules! ffi_safe_str {
    ($f:expr) => {{
        let res = panic::catch_unwind(panic::AssertUnwindSafe($f));
        match res {
            Ok(Ok(s)) => CString::new(s).unwrap().into_raw(),
            Ok(Err(e)) => CString::new(format!("ERR:{}", e)).unwrap().into_raw(),
            Err(_) => CString::new("PANIC").unwrap().into_raw(),
        }
    }};
}

// ---------------- 文档解析 FFI ----------------
#[no_mangle]
pub extern "C" fn luna_parse_document(path: *const c_char, chunk_size: usize) -> *mut c_char {
    ffi_safe_str! {
        || -> Result<String, String> {
            let path_str = unsafe { CStr::from_ptr(path).to_str().map_err(|e| e.to_string())? };
            let chunks = parser::parse_and_chunk(path_str, chunk_size)?;
            let output = serde_json::json!({
                "status": "ok",
                "chunks": chunks,
                "count": chunks.len()
            });
            serde_json::to_string(&output).map_err(|e| e.to_string())
        }
    }
}

// ---------------- 向量索引 FFI ----------------
#[no_mangle]
pub extern "C" fn luna_index_create(dimension: usize) -> *mut c_void {
    panic::catch_unwind(panic::AssertUnwindSafe(|| {
        Box::into_raw(Box::new(index::FlatIndex::new(dimension))) as *mut c_void
    }))
    .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn luna_index_add(
    idx_ptr: *mut c_void,
    vec_json: *const c_char,
    meta_json: *const c_char,
) -> *mut c_char {
    ffi_safe_str! {
        || -> Result<String, String> {
            let idx = unsafe { &mut *(idx_ptr as *mut index::FlatIndex) };
            let vec: Vec<f32> = serde_json::from_str(unsafe { CStr::from_ptr(vec_json).to_str().map_err(|e| e.to_string())? }).map_err(|e| e.to_string())?;
            let meta = unsafe { CStr::from_ptr(meta_json).to_str().map_err(|e| e.to_string())? }.to_string();
            idx.add(vec, meta).map_err(|e| e.to_string())?;
            Ok("ok".to_string())
        }
    }
}

#[no_mangle]
pub extern "C" fn luna_index_search(
    idx_ptr: *mut c_void,
    query_json: *const c_char,
    top_k: usize,
) -> *mut c_char {
    ffi_safe_str! {
        || -> Result<String, String> {
            let idx = unsafe { &*(idx_ptr as *mut index::FlatIndex) };
            let query: Vec<f32> = serde_json::from_str(unsafe { CStr::from_ptr(query_json).to_str().map_err(|e| e.to_string())? }).map_err(|e| e.to_string())?;
            let results = idx.search(&query, top_k).map_err(|e| e.to_string())?;

            let out: Vec<_> = results.into_iter().map(|(i, score)| {
                let meta_val: serde_json::Value = serde_json::from_str(&idx.entries[i].meta).unwrap_or_default();
                serde_json::json!({ "id": i, "score": score, "metadata": meta_val })
            }).collect();

            serde_json::to_string(&out).map_err(|e| e.to_string())
        }
    }
}

#[no_mangle]
pub extern "C" fn luna_index_save(idx_ptr: *mut c_void, path: *const c_char) -> *mut c_char {
    ffi_safe_str! {
        || -> Result<String, String> {
            let idx = unsafe { &*(idx_ptr as *mut index::FlatIndex) };
            let path_str = unsafe { CStr::from_ptr(path).to_str().map_err(|e| e.to_string())? };
            idx.save(path_str).map_err(|e| e.to_string())?;
            Ok("ok".to_string())
        }
    }
}

#[no_mangle]
pub extern "C" fn luna_index_free(idx_ptr: *mut c_void) {
    if idx_ptr.is_null() {
        return;
    }
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| unsafe {
        drop(Box::from_raw(idx_ptr as *mut index::FlatIndex));
    }));
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
