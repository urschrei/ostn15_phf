use phf_codegen;
use std::env;
const GENERATED_FILE: &'static str = "src/ostn15.rs";

#[derive(Debug)]
struct Shift {
    key: i32,
    eastings: f64,
    northings: f64,
    height: f64,
}

// run build script like BUILD_ENABLED=1 cargo build
fn main() {
    let build_enabled = env::var("BUILD_ENABLED").map(|v| v == "1").unwrap_or(false); // don't run by default
    if build_enabled {
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        if target_arch != "aarch64" {
            eprintln!("Compatible architecture found for build script. Proceeding.");
            use rusqlite::Connection;
            use std::fs::File;
            use std::io::prelude::*;
            use std::io::BufWriter;
            let conn = Connection::open("src/OSTN15.db").unwrap();

            let mut outfile = BufWriter::new(File::create(GENERATED_FILE).unwrap());
            write!(outfile, "static OSTN15: phf::Map<i32, (f64, f64, f64)> = ").unwrap();
            let mut stmt = conn.prepare("SELECT * FROM ostn15").unwrap();
            let ostn15_iter = stmt
                .query_map([], |row| {
                    Ok(Shift {
                        key: row.get(0).unwrap(),
                        eastings: row.get(1).unwrap(),
                        northings: row.get(2).unwrap(),
                        height: row.get(3).unwrap(),
                    })
                })
                .unwrap();
            let mut keys = vec![];
            let mut values = vec![];
            for each in ostn15_iter {
                let record = each.unwrap();
                keys.push(record.key);
                values.push((
                    record.eastings as f64,
                    record.northings as f64,
                    record.height as f64,
                ));
            }
            let results: Vec<_> = keys.iter().zip(values.iter()).collect();
            let mut map = phf_codegen::Map::<i32>::new();
            for &(ref key, val) in &results {
                map.entry(
                    **key,
                    &format!("({:.3}, {:.3}, {:.3})", val.0, val.1, val.2),
                );
            }
            writeln!(&mut outfile, "{}", map.build()).unwrap();
            writeln!(&mut outfile, ";").unwrap();
        }
    }
}
