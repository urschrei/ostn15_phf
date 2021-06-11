#![doc(html_root_url = "https://urschrei.github.io/ostn15_phf/")]
//! Look up OSTN15 adjustments for transforming ETRS89 Eastings and Northings
//! to OSGB36 Eastings and Northings

use std::f64;
const NAN: f64 = f64::NAN;

use phf;
include!("ostn15.rs");

use libc::c_double;

/// Return a 3-tuple of adjustments which convert ETRS89 Eastings and Northings
/// to OSGB36 Eastings, Northings, and Orthometric height
fn get_shifts(tup: (i32, i32)) -> (f64, f64, f64) {
    // look up the shifts, or return NAN
    let key = tup.0 + (tup.1 * 701) + 1;

    match ostn15_lookup(&key) {
        Some(res) => (res.0, res.1, res.2),
        None => (NAN, NAN, NAN),
    }
}

#[repr(C)]
/// Incoming ETRS89 kilometer-grid references
pub struct GridRefs {
    pub easting: i32,
    pub northing: i32,
}

#[repr(C)]
/// Outgoing OSTN15 Easting, Northing, and height adjustments
pub struct Adjustment {
    pub x_shift: c_double,
    pub y_shift: c_double,
    pub z_shift: c_double,
}

// From and Into traits for GridRefs
impl From<(i32, i32)> for GridRefs {
    fn from(gr: (i32, i32)) -> GridRefs {
        GridRefs {
            easting: gr.0,
            northing: gr.1,
        }
    }
}

impl From<GridRefs> for (i32, i32) {
    fn from(gr: GridRefs) -> (i32, i32) {
        (gr.easting, gr.northing)
    }
}

// From and Into traits for Adjustment
impl From<(f64, f64, f64)> for Adjustment {
    fn from(adj: (f64, f64, f64)) -> Adjustment {
        Adjustment {
            x_shift: adj.0,
            y_shift: adj.1,
            z_shift: adj.2,
        }
    }
}

impl From<Adjustment> for (f64, f64, f64) {
    fn from(adj: Adjustment) -> (f64, f64, f64) {
        (adj.x_shift, adj.y_shift, adj.z_shift)
    }
}

/// FFI function returning a 3-tuple of Easting, Northing, and height adjustments, for use in transforming
/// ETRS89 Eastings and Northings to OSGB36 Eastings, Northings.  
/// The argument is a Struct containing kilometer-grid references of the ETRS89 Northings and Eastings you wish to convert
///
/// # Examples
///
/// ```python
/// # Python example using ctypes
/// import sys, ctypes
/// from ctypes import c_int32, c_double, Structure
///
///
/// class GridRefs(Structure):
///     _fields_ = [("eastings", c_int32),
///                 ("northings", c_int32)]
///
///     def __str__(self):
///         return "({},{})".format(self.eastings, self.northings)
///
///
/// class Shifts(Structure):
///     _fields_ = [("x_shift", c_double),
///                 ("y_shift", c_double),
///                 ("z_shift", c_double)]
///
///     def __str__(self):
///         return "({}, {}, {})".format(self.x_shift, self.y_shift, self.z_shift)
///
///
/// prefix = {'win32': ''}.get(sys.platform, 'lib')
/// extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
/// lib = ctypes.cdll.LoadLibrary(prefix + "ostn02_phf" + extension)
///
/// lib.get_shifts_ffi.argtypes = (GridRefs,)
/// lib.get_shifts_ffi.restype = Shifts
///
/// tup = GridRefs(651, 313)
///
/// # Should return (102.775, -78.244, 44.252)
/// print lib.get_shifts_ffi(tup)
/// ```
#[no_mangle]
pub extern "C" fn get_shifts_ffi(gr: GridRefs) -> Adjustment {
    get_shifts(gr.into()).into()
}

/// Return a 3-tuple of Easting, Northing, and height adjustments, for use in transforming
/// ETRS89 Eastings and Northings to OSGB36 Eastings, Northings.  
///
/// # Examples
///
/// ```
/// use ostn15_phf::ostn15_lookup;
///
/// // Caister Tower Eastings and Northings: 651307.003, 313255.686
/// let e_grid = (651307.003 / 1000.) as i32;
/// let n_grid = (313255.686 / 1000.) as i32;
/// let key = e_grid + (n_grid * 701) +1;
/// // key should be 220065
/// let result = ostn15_lookup(&key).unwrap();
/// // result should be (102.787, -78.242, 44.236)
/// assert_eq!(result, (102.787, -78.242, 44.236));
/// // remember that the actual adjustment for a coordinate is a bilinear transform, using a square
/// ```
pub fn ostn15_lookup(key: &i32) -> Option<(f64, f64, f64)> {
    OSTN15.get(key).cloned()
}

#[test]
fn test_internal_ffi() {
    assert_eq!((102.787, -78.242, 44.236), get_shifts((651, 313)));
}

#[test]
fn test_ffi() {
    let gr = GridRefs {
        easting: 651,
        northing: 313,
    };
    assert_eq!((102.787, -78.242, 44.236), get_shifts_ffi(gr).into());
}
