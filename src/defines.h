#pragma once

#include <config.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

#if UINTPTR_MAX == UINT64_MAX
typedef u64 usize;
typedef i64 isize;
#elif UINTPTR_MAX == UINT32_MAX
typedef u32 usize;
typedef i32 isize;
#else
#error "Unsupported pointer width"
#endif

static inline usize neosh_max(usize a, usize b) { return a > b ? a : b; }

_Noreturn static inline void neosh_panic_impl(const char *file, i32 line,
                                              const char *func,
                                              const char *message) {
  fprintf(stderr, "Panic at %s:%d in %s: %s\n", file, line, func, message);
  abort();
}

#define neosh_panic(msg) neosh_panic_impl(__FILE__, __LINE__, __func__, msg)

#if NEOSH_DEBUG
#define neosh_assert(expr)                                                     \
  do {                                                                         \
    if (!(expr))                                                               \
      neosh_panic("Assertion failed: " #expr);                                 \
  } while (0)
#else
#define neosh_assert(expr) ((void)0)
#endif
