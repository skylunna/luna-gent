package ffi

/*
#cgo CFLAGS: -I${SRCDIR}/../../rust/target/include
#cgo LDFLAGS: -L${SRCDIR}/../../rust/target/release -lluna_core
#include <luna_core.h>
#include <stdlib.h>
*/
import "C"

import (
	"errors"
	"unsafe"
)

// 调用 Rust 函数
func Greet(name string) (string, error) {
	// 字符串转为 C 格式
	cName := C.CString(name)
	if cName == nil {
		return "", errors.New("alloc cstring failed")
	}
	defer C.free(unsafe.Pointer(cName))

	// 调用 Rust FFI 函数
	cMsg := C.luna_greet(cName)
	if cMsg == nil {
		return "", errors.New("rust returned null")
	}
	defer C.luna_free_string(cMsg)

	// 转回 Go 字符串
	msg := C.GoString(cMsg)

	if msg == "RUST_PANIC" {
		return "", errors.New("rust panic in luna_core")
	}

	return msg, nil
}
