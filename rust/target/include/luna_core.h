#ifndef LUNA_CORE_H
#define LUNA_CORE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * 安全包装, 防止 Rust Panic 穿透 CGO 导致 Go 进程崩溃
 */
char *luna_greet(const char *name);

/**
 * Go 侧必须调用此函数释放 Rust 分配的 C 字符串
 */
void luna_free_string(char *ptr);

#endif  /* LUNA_CORE_H */
