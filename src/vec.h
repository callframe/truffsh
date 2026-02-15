#pragma once

#include "defines.h"

#define NEOSH_VEC_INITIAL 4
#define NEOSH_VEC_GROWTH 2

struct vec_s {
  u8 *elems;
  usize esize;
  usize ecap, elen;
};

#define neosh_vec_init(etype)                                                  \
  (struct vec_s){.elems = NULL, .esize = sizeof(etype), .ecap = 0, .elen = 0}

void neosh_vec_push_back(struct vec_s *vec, const u8 *elem);

void neosh_vec_deinit(struct vec_s *vec);
