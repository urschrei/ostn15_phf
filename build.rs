use lzma_rust2::XzReader;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write as _};
use std::path::PathBuf;

/// Path of the xz-compressed OSTN15 shifts data file, relative to the project root.
///
/// This is the original name from the OSTN15 developer pack from OS, with .xz appended.
const DATA_FILE_PATH: &str = "OSTN15_OSGM15_DataFile.txt.xz";

/// Write a generated PHF map to a generated.rs file when building.
fn main() {
    let data_file = open_xz_file(DATA_FILE_PATH);
    let mut map_builder = phf_codegen::Map::<i32>::new();
    // Mutable string to avoid 1M short-lived allocations via format! macro.
    let mut string_buf = String::with_capacity(64);
    // Skip header line
    let mut lines = data_file.lines().skip(1);
    while let Some(Ok(line)) = lines.next() {
        let (point_id, (e, n, h)) = parse_line(&line);
        // Note the data file shift strings are valid Rust float literals. They are formatted
        // individually (not as a tuple) to prevent the inclusion of quotes.
        write!(&mut string_buf, "({e}, {n}, {h})").expect("Failed to format tuple string");
        map_builder.entry(point_id, &string_buf);
        string_buf.clear();
    }
    let generated_map = map_builder.build();
    write_module(generated_map);

    // Instruct Cargo to only rerun the build script if the data file itself changes (it will
    // automatically re-run if the build script itself changes).
    println!("cargo::rerun-if-changed={}", DATA_FILE_PATH);
}

/// Decompress an xz-encoded file, with buffering.
fn open_xz_file(path: &str) -> BufReader<XzReader<File>> {
    let data_file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open data file");
    let xz_reader = XzReader::new(data_file, false);
    BufReader::new(xz_reader)
}

/// Extract the Point_ID and shifts from a line in the data file.
///
/// The returned tuple is the Point_ID and a triple of the easting, northing and height shifts (in
/// that order). The shifts are returned as &str because the shifts in the file are valid as Rust
/// floats, so parsing them as f64 only to format them as strings is unnecessary extra work.
fn parse_line(line: &str) -> (i32, (&str, &str, &str)) {
    let mut parts = line.split(',');
    let point_id: i32 = parts
        .next()
        .expect("Point ID missing")
        .parse()
        .expect("Failed to parse point ID as i32");
    let _easting = parts.next();
    let _northing = parts.next();
    let easting_shift = parts.next().expect("Easting shift missing");
    let northing_shift = parts.next().expect("Northing shift missing");
    let height_shift = parts.next().expect("Height shift missing");
    (point_id, (easting_shift, northing_shift, height_shift))
}

/// Write the generated map as a static to a generated.rs file in the build directory.
fn write_module(map: phf_codegen::DisplayMap<i32>) {
    // OUT_DIR is set by Cargo for build scripts.
    let out_dir = std::env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR variable not set. Not running as a build script?");
    // Rust module that will contain the generated map.
    let out_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_dir.join("generated.rs"))
        .expect("Failed to open generated.rs in OUT_DIR for writing");

    let mut writer = BufWriter::new(out_file);
    writeln!(writer, "{TEMPLATE_HEAD}{map};{TEMPLATE_TAIL}")
        .expect("Failed to write template to output file.");
}

const TEMPLATE_HEAD: &str = "\
mod generated {
    #![allow(clippy::all)]
    pub(super) static OSTN15: phf::Map<i32, (f64, f64, f64)> = ";

const TEMPLATE_TAIL: &str = "\n}";
