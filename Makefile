.PHONY: build clean

ROOT_DIR := $(shell pwd)

build:
	@echo "🛠️  Building luna-gent (Go + Rust FFI)..."
	@mkdir -p rust/target/include
	# 1. 生成 C 头文件
	@cd rust && cbindgen --config cbindgen.toml --crate luna-core --output $(ROOT_DIR)/rust/target/include/luna_core.h
	# 2. 编译 Rust 静态库
	@cd rust && cargo build --release
	# 3. 编译 Go 二进制 (启用 CGO)
	@CGO_ENABLED=1 \
	 CGO_LDFLAGS="-L$(ROOT_DIR)/rust/target/release" \
	 go build -ldflags="-s -w" -o bin/luna-gent ./cmd/luna-gent
	@echo "✅  Build success: bin/luna-gent"

clean:
	rm -rf bin/ rust/target/