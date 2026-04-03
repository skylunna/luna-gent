package ffi

/*
#cgo CFLAGS: -I${SRCDIR}/../../rust/target/include
#cgo LDFLAGS: -L${SRCDIR}/../../rust/target/release -lluna_core
#include <stdlib.h>
#include "luna_core.h"
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"unsafe"
)

type ParseResponse struct {
	Status  string      `json:"status"`
	Message string      `json:"message,omitempty"`
	Chunks  []ChunkData `json:"chunks,omitempty"`
	Count   int         `json:"count"`
}

type ChunkData struct {
	ID       int               `json:"id"`
	Content  string            `json:"content"`
	Metadata map[string]string `json:"metadata"`
}

func ParseDocument(path string, chunkSize int) ([]ChunkData, error) {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	cRes := C.luna_parse_document(cPath, C.size_t(chunkSize))
	defer C.luna_free_string(cRes)

	jsonStr := C.GoString(cRes)
	var resp ParseResponse
	if err := json.Unmarshal([]byte(jsonStr), &resp); err != nil {
		return nil, fmt.Errorf("failed to parse Rust JSON response: %w", err)
	}

	if resp.Status != "ok" {
		return nil, fmt.Errorf("rust parse error: %s", resp.Message)
	}
	return resp.Chunks, nil
}
