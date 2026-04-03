use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::catch_unwind;

/// 安全包装, 防止 Rust Panic 穿透 CGO 导致 Go 进程崩溃
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn luna_greet(name: *const c_char) -> *mut c_char {
    let res = catch_unwind(|| {
        // 指针有效性由调用者（Go）保证，此处安全解引用
        let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap_or("World") };
        format!("🌙 Hello from Luna-Core, {}! FFI Bridge Active.", name_str)
    });

    match res {
        Ok(msg) => CString::new(msg).unwrap().into_raw(),
        Err(_) => CString::new("RUST_PANIC").unwrap().into_raw(),
    }
}

/// Go 侧必须调用此函数释放 Rust 分配的 C 字符串
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn luna_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}
