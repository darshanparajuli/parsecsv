use parsecsv_proc_macro::CsvSerialize;
use parsecsv_serializer::Serializer;

#[derive(CsvSerialize)]
struct A {
    id: u64,
    vec: Vec<u32>,
    string: String,
    b: B,
    c: C,
}

#[derive(CsvSerialize)]
struct B {
    a: u32,
}

#[derive(CsvSerialize)]
struct C(u32);

fn main() {
    let a = A {
        id: 1,
        vec: vec![1, 2, 3],
        string: "foo hello world".into(),
        b: B { a: 12 },
        c: C(13),
    };
    println!("{}", parsecsv::to_string(&a).unwrap());
}
