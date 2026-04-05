package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math"
	"math/rand/v2"
	"os"

	"github.com/skylunna/luna-gent/pkg/ffi" // 请确保与 go.mod 的 module 名称一致
)

func main() {
	fmt.Println("🌙 luna-gent v0.1.4 | Vector Index & Search Test")

	// 1. 解析文档
	testFile := "luna_test.md"
	content := `# 架构设计
Go 负责调度，Rust 负责计算。
向量检索采用 Flat Index + 余弦相似度。

# 性能优化
rkyv 序列化支持零拷贝加载。
内存安全由 Rust 借用检查器保障。

# 下一步
接入真实 Embedding API 网关。
`
	os.WriteFile(testFile, []byte(content), 0644)
	defer os.Remove(testFile)

	chunks, err := ffi.ParseDocument(testFile, 100)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("✅ Parsed %d chunks\n", len(chunks))

	// 2. 建索引 (模拟 Embedding 向量)
	dim := 128
	idx := ffi.IndexCreate(dim)
	defer ffi.IndexFree(idx)

	fmt.Println("🔧 Building index with mock embeddings...")
	for _, c := range chunks {
		vec := mockEmbed(dim)
		vecJSON, _ := json.Marshal(vec)
		metaJSON, _ := json.Marshal(c.Metadata)
		if err := ffi.IndexAdd(idx, string(vecJSON), string(metaJSON)); err != nil {
			log.Fatal(err)
		}
	}

	// 3. 语义检索
	query := mockEmbed(dim)
	queryJSON, _ := json.Marshal(query)
	results, err := ffi.IndexSearch(idx, string(queryJSON), 2)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("🔍 Top-2 Results:\n")
	for i, r := range results {
		fmt.Printf("  [%d] Score: %.4f | Meta: %v\n", i, r.Score, r.Metadata)
	}

	// 4. 持久化验证
	idxPath := "test_index.rkyv"
	defer os.Remove(idxPath)
	if err := ffi.IndexSave(idx, idxPath); err != nil {
		log.Fatal(err)
	}
	fmt.Println("💾 Index saved successfully (verify with ls -lh)")
}

// 生成归一化随机向量（模拟 LLM Embedding）
func mockEmbed(dim int) []float32 {
	v := make([]float32, dim)
	var norm float32
	for i := range v {
		v[i] = rand.Float32()
		norm += v[i] * v[i]
	}
	norm = float32(math.Sqrt(float64(norm)))
	for i := range v {
		v[i] /= norm
	}
	return v
}
