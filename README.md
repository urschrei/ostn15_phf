[![Build Status](https://travis-ci.org/urschrei/ostn15_phf.png?branch=master)](https://travis-ci.org/urschrei/ostn15_phf) [![](https://img.shields.io/crates/v/lonlat_bng.svg)](https://crates.io/crates/OSTN02_PHF) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](license.txt)  

# Description
A Rust Crate which provides fast lookup of [OSTN15 adjustments](https://www.ordnancesurvey.co.uk/business-and-government/help-and-support/navigation-technology/os-net/surveying.html), for the conversion of ETRS89 grid coordinates to OSGB36.  

# Rust Crate Example
``` rust
use ostn15_phf::ostn15_lookup;
// Caister Tower Eastings and Northings: 651307.003, 313255.686
let e_grid = (651307.003 / 1000.) as i32;
let n_grid = (313255.686 / 1000.) as i32;
let key = e_grid + (n_grid * 701) + 1
// key is 220065
// don't use unwrap() in production
let result = ostn15_lookup(&key).unwrap();
// result should be (102.787, -78.242, 44.236)
assert_eq!(result, (102.787, -78.242, 44.236));
// remember that the actual adjustment for a coordinate is a bilinear transform, using a square
```

# FFI Examples
## Python
``` python
import sys, ctypes
from ctypes import c_int32, c_double, Structure


class GridRefs(Structure):
    _fields_ = [("eastings", c_int32),
                ("northings", c_int32)]

    def __str__(self):
        return "({}, {})".format(self.eastings, self.northings)


class Shifts(Structure):
    _fields_ = [("x_shift", c_double),
                ("y_shift", c_double),
                ("z_shift", c_double)]

    def __str__(self):
        return "({}, {}, {})".format(self.x_shift, self.y_shift, self.z_shift)


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + "ostn15_phf" + extension)

lib.get_shifts_ffi.argtypes = (GridRefs,)
lib.get_shifts_ffi.restype = Shifts

result = GridRefs(651, 313)

print(lib.get_shifts_ffi(result))
```

## C
``` c
// compile with e.g. `clang -lostn15_phf -L target/release -o ostn15_shifts  src/ostn15.c` from project root
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
```

# Building the Shared Library
- Ensure that [Rust](https://www.rust-lang.org/downloads.html) is installed
- Clone this repo
- In the repo root, run `cargo build --release`
- The dylib or DLL will be available as `target/release/libostn15_phf.{dylib, dll}`
- If you need to build a `.so` for Linux:
    1. `ar -x target/release/liblonlat_bng.a`
    2. `gcc -shared *.o -o target/release/libostn15_phf.so`

# License
[MIT](LICENSE)  

This software makes use of OSTN15 data, which is © Crown copyright, Ordnance Survey and the Ministry of Defence (MOD) 2016. All rights reserved. Provided under the BSD 2-clause [license](OSTN15_license.txt).
