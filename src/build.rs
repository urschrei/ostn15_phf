extern crate phf_codegen;
extern crate rusqlite;
use rusqlite::Connection;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

const GENERATED_FILE: &'static str = "src/ostn15.rs";

#[derive(Debug)]
struct Shift {
    key: i32,
    eastings: f64,
    northings: f64,
    height: f64,
}

fn main() {
    let conn = Connection::open("src/OSTN15.db").unwrap();

    let mut outfile = BufWriter::new(File::create(GENERATED_FILE).unwrap());
    write!(outfile,
           "static OSTN15: phf::Map<i32, (f64, f64, f64)> = ")
        .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM ostn15 WHERE key =  '220065' LIMIT 100")
                       .unwrap();
    let ostn15_iter = stmt.query_map(&[], |row| {
                                  Shift {
                                      key: row.get(0),
                                      eastings: row.get(1),
                                      northings: row.get(2),
                                      height: row.get(3),
                                  }
                              })
                              .unwrap();

    let mut keys = vec![];
    let mut values = vec![];
    for each in ostn15_iter {
        let record = each.unwrap();
        keys.push(record.key);
        values.push((record.eastings as f64,
                     record.northings as f64,
                     record.height as f64));
    }
    let results: Vec<_> = keys.iter().zip(values.iter()).collect();
    let mut map = phf_codegen::Map::<i32>::new();
    for &(ref key, val) in &results {
        map.entry(**key, &format!("{:?}", val));
    }
    map.build(&mut outfile).unwrap();
    writeln!(outfile, ";").unwrap();
}
