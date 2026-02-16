#pragma once

#include <defines.h>
#include <vec.h>

struct stream_s {
  i32 fd;
  u8 *buf;
  usize bufsize;
  usize start;
};
