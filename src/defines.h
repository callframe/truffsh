#pragma once

#include <stdint.h>

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
