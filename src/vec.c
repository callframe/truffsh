#include "mimalloc.h"
#include <defines.h>
#include <string.h>
#include <vec.h>

static inline usize neosh_vec_size(const struct vec_s *vec, usize nelems) {
  neosh_assert(vec != NULL);
  return nelems * vec->esize;
}

static void neosh_vec_grow(struct vec_s *vec, usize new_nelems) {
  usize grown_ecap = vec->ecap * NEOSH_VEC_GROWTH;
  usize required = neosh_max(new_nelems, NEOSH_VEC_INITIAL);

  usize new_ecap = neosh_max(grown_ecap, required);
  usize new_bytes = neosh_vec_size(vec, new_ecap);

  u8 *new_elems = mi_malloc(new_bytes);

  if (vec->elems == NULL) {
    vec->elems = new_elems;
    vec->ecap = new_ecap;
    return;
  }

  memcpy(new_elems, vec->elems, neosh_vec_size(vec, vec->elen));
  mi_free(vec->elems);
  vec->elems = new_elems;
  vec->ecap = new_ecap;
}

/* static void neosh_vec_shrink(struct vec_s *vec) {
  usize shrink_ecap = vec->ecap / NEOSH_VEC_SHRINK;
  if (shrink_ecap == 0)
    return;
  if (vec->elen > shrink_ecap)
    return;

  usize new_bytes = neosh_vec_size(vec, shrink_ecap);
  u8 *new_elems = mi_malloc(new_bytes);

  memcpy(new_elems, vec->elems, neosh_vec_size(vec, vec->elen));
  mi_free(vec->elems);

  vec->elems = new_elems;
  vec->ecap = shrink_ecap;
} */

static inline u8 *neosh_vec_ptr(struct vec_s *vec, usize index) {
  neosh_assert(vec != NULL);
  return vec->elems + neosh_vec_size(vec, index);
}

void neosh_vec_push_back(struct vec_s *vec, const u8 *elem) {
  neosh_assert(vec != NULL);
  neosh_assert(elem != NULL);

  if (vec->elen == vec->ecap)
    neosh_vec_grow(vec, vec->elen + 1);

  u8 *dest = neosh_vec_ptr(vec, vec->elen);
  memcpy(dest, elem, vec->esize);
  vec->elen++;
}

void neosh_vec_deinit(struct vec_s *vec) {
  neosh_assert(vec != NULL);

  if (vec->elems == NULL)
    return;

  mi_free(vec->elems);
  vec->elems = NULL;
  vec->ecap = 0;
  vec->elen = 0;
}
