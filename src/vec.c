#include <defines.h>
#include <stdlib.h>
#include <string.h>
#include <vec.h>

static void neosh_vec_grow(struct vec_s *vec, usize new_nelems) {
  usize grown_ecap = vec->ecap * NEOSH_VEC_GROWTH;
  usize required = neosh_max(new_nelems, NEOSH_VEC_INITIAL);
  usize new_ecap = neosh_max(grown_ecap, required);

  usize old_bytes = vec->elen * vec->esize;
  usize new_bytes = new_ecap * vec->esize;

  u8 *new_elems = malloc(new_bytes);
  if (old_bytes > 0)
    memcpy(new_elems, vec->elems, old_bytes);
  free(vec->elems);

  vec->elems = new_elems;
  vec->ecap = new_ecap;
}

static inline u8 *neosh_vec_ptr_at(struct vec_s *vec, usize index) {
  neosh_assert(vec != NULL);
  return vec->elems + index * vec->esize;
}

void neosh_vec_push_back(struct vec_s *vec, const u8 *elem) {
  neosh_assert(vec != NULL);
  neosh_assert(elem != NULL);

  if (vec->elen == vec->ecap)
    neosh_vec_grow(vec, vec->elen + 1);

  u8 *dest = neosh_vec_ptr_at(vec, vec->elen);
  memcpy(dest, elem, vec->esize);
  vec->elen++;
}

void neosh_vec_deinit(struct vec_s *vec) {
  neosh_assert(vec != NULL);

  if (vec->elems == NULL)
    return;

  free(vec->elems);
  vec->elems = NULL;
  vec->ecap = 0;
  vec->elen = 0;
}
