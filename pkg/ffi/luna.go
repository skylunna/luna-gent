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

type ChunkData struct {
	ID       int               `json:"id"`
	Content  string            `json:"content"`
	Metadata map[string]string `json:"meta"`
}

type SearchResult struct {
	ID       int               `json:"id"`
	Score    float64           `json:"score"`
	Metadata map[string]string `json:"metadata"`
}

// 解析层
func ParseDocument(path string, chunkSize int) ([]ChunkData, error) {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	cRes := C.luna_parse_document(cPath, C.size_t(chunkSize))
	defer C.luna_free_string(cRes)

	var resp struct {
		Status string      `json:"status"`
		Chunks []ChunkData `json:"chunks"`
	}
	if err := json.Unmarshal([]byte(C.GoString(cRes)), &resp); err != nil {
		return nil, err
	}
	if resp.Status != "ok" {
		return nil, fmt.Errorf("parse failed")
	}
	return resp.Chunks, nil
}

// 索引层
func IndexCreate(dimension int) unsafe.Pointer {
	return C.luna_index_create(C.size_t(dimension))
}

func IndexAdd(idx unsafe.Pointer, vectorJSON string, metaJSON string) error {
	cVec := C.CString(vectorJSON)
	defer C.free(unsafe.Pointer(cVec))
	cMeta := C.CString(metaJSON)
	defer C.free(unsafe.Pointer(cMeta))

	cRes := C.luna_index_add(idx, cVec, cMeta)
	defer C.luna_free_string(cRes)
	if s := C.GoString(cRes); s != "ok" {
		return fmt.Errorf("index add failed: %s", s)
	}
	return nil
}

func IndexSearch(idx unsafe.Pointer, queryJSON string, topK int) ([]SearchResult, error) {
	cQuery := C.CString(queryJSON)
	defer C.free(unsafe.Pointer(cQuery))
	cRes := C.luna_index_search(idx, cQuery, C.size_t(topK))
	defer C.luna_free_string(cRes)

	var results []SearchResult
	if err := json.Unmarshal([]byte(C.GoString(cRes)), &results); err != nil {
		return nil, err
	}
	return results, nil
}

func IndexSave(idx unsafe.Pointer, path string) error {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	cRes := C.luna_index_save(idx, cPath)
	defer C.luna_free_string(cRes)
	if s := C.GoString(cRes); s != "ok" {
		return fmt.Errorf("save failed: %s", s)
	}
	return nil
}

func IndexFree(idx unsafe.Pointer) {
	if idx != nil {
		C.luna_index_free(idx)
	}
}
