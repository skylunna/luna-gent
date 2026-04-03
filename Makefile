.PHONY: build clean

# 编译 Rust 静态库
rust:
	cd rust && cargo build --release

# 编译 Go 二进制（暂不链接 Rust，先跑通结构）
go:
	CGO_ENABLED=0 go build -ldflags="-s -w" -o bin/luna-gent ./cmd/luna-gent

# 完整构建（后续启用 CGO）
build: rust go

clean:
	rm -rf bin/ rust/target/