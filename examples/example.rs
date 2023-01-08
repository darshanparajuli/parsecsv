use parsecsv_proc_macro::CsvSerialize;
use parsecsv_serializer::Serializer;
use std::io::BufWriter;

#[derive(CsvSerialize)]
struct B {
    a: u32,
}

#[derive(CsvSerialize)]
struct A {
    a: u64,
    b: u64,
    c: u64,
    d: Vec<u32>,
    e: String,
    f: C,
}

#[derive(CsvSerialize)]
struct C(u32);

fn main() {
    let a = A { a: 1, b: 2, c: 3, d: vec![1,2,3], e: "foo hello world".into(), f: C(13) };
    let mut writer = BufWriter::new(Vec::new());

    // Write
    a.serialize(&mut writer).unwrap();

    let s = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    s.to_string();
    println!("{}", s);
}
