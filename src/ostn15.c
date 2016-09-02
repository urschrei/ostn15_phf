// compile with `clang -lostn15_phf -L target/release -o ostn15_shifts  src/ostn15.c` from project root
// run with `LD_LIBRARY_PATH=target/release ./ostn15_shifts` from project root

#include <stdio.h>
#include <stdint.h>

typedef struct {
  int32_t easting;
  int32_t northing;
} gridrefs;

typedef struct {
  double x_shift;
  double y_shift;
  double z_shift;
} adjustment;

extern adjustment get_shifts_ffi(gridrefs);

int main(void) {
  gridrefs initial = { .easting = 651, .northing = 313 };
  adjustment adj = get_shifts_ffi(initial);
  printf("(%f, %f, %f)\n", adj.x_shift, adj.y_shift, adj.z_shift);
  return 0;
}
