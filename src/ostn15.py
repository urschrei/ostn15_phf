#!/usr/bin/env python

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


prefix = {"win32": ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary("target/release/" + prefix + "ostn15_phf" + extension)
lib.get_shifts_ffi.argtypes = (GridRefs,)
lib.get_shifts_ffi.restype = Shifts
result = GridRefs(651, 313)
# Should return (102.787, -78.242, 44.236)
print(lib.get_shifts_ffi(result))
