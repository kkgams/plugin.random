#include "random_provider.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  random_provider_list_u8_t first, second;
  exports_gams_random_seeded_random_set_seed(123);
  exports_wasi_random_random_get_random_bytes(16, &first);
  exports_gams_random_seeded_random_set_seed(123);
  exports_wasi_random_random_get_random_bytes(16, &second);
  assert(first.len == 16 && second.len == 16);
  assert(memcmp(first.ptr, second.ptr, 16) == 0);
  assert(exports_gams_random_seeded_random_get_seed() == 123);
  free(first.ptr);
  free(second.ptr);
  return 0;
}
