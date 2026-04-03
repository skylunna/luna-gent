package main

import (
	"fmt"
	"log"

	"github.com/skylunna/luna-gent/pkg/ffi" // 请确保与 go.mod 的 module 名称一致
)

func main() {
	fmt.Println("🌙 luna-gent v0.1.2 | FFI Bridge Test")

	res, err := ffi.Greet("Developer")
	if err != nil {
		log.Fatalf("FFI call failed: %v", err)
	}
	fmt.Println(res)
}
