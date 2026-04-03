//! Luna-Gent Core - Rust 高性能引擎

/// 获取版本号
#[no_mangle]
pub extern "C" fn lunagent_version() -> *const std::os::raw::c_char {
    b"Luna-Gent Rust Core v0.1.0\0".as_ptr() as *const _
}

/// 文档解析（未来实现）
#[no_mangle]
pub extern "C" fn parse_document() -> i32 {
    0
}

/// 向量检索（未来实现）
#[no_mangle]
pub extern "C" fn retrieve() -> i32 {
    0
}