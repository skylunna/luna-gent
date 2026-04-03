package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/skylunna/luna-gent/pkg/ffi" // 请确保与 go.mod 的 module 名称一致
)

func main() {
	fmt.Println("🌙 luna-gent v0.1.3 | Document Parsing Test")

	// 自动生成测试文件，跑完自动清理
	testFile := "luna_test_sample.md"
	testContent := `# Luna-Gent 测试文档
这是用于验证 FFI 解析层的示例文档。
我们将测试多段落切割与元数据绑定功能。

## 架构设计
Go 负责调度与交互，Rust 负责高性能计算。
这是第一段的补充说明，用于测试长度累积逻辑。

## 下一阶段
向量检索与 Embedding 将在后续迭代中接入。
目前我们只关注文档解析与分块是否正确返回。
`
	if err := os.WriteFile(testFile, []byte(testContent), 0644); err != nil {
		log.Fatalf("❌ Failed to create test file: %v", err)
	}
	defer os.Remove(testFile)

	fmt.Printf("📄 Parsing: %s (chunk_size: 120 chars)\n", testFile)
	chunks, err := ffi.ParseDocument(testFile, 120)
	if err != nil {
		log.Fatalf("❌ Parse failed: %v", err)
	}

	fmt.Printf("✅ Success! Found %d chunks:\n", len(chunks))
	for i, c := range chunks {
		preview := truncate(c.Content, 60)
		fmt.Printf("  [Chunk %d] ID: %d | Len: %d | Source: %s\n    → %s\n",
			i+1, c.ID, len(c.Content), c.Metadata["source"], preview)
	}
}

func truncate(s string, max int) string {
	s = strings.TrimSpace(s)
	if len(s) > max {
		return s[:max] + "..."
	}
	return s
}
