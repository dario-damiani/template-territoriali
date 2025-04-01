use std::{fs::File, io::{Read, Write}};

fn risolvi(dati: &mut Vec<i64>) -> i64 {
    // leggi input
}

fn main() {
    let mut input = File::open("{nomebreve}_input_1.txt").unwrap();
    let mut string = String::new();
    input.read_to_string(&mut string).unwrap();
    let mut output = File::create("output.txt").unwrap();
    let mut data = string.split_whitespace().map(|text| {
        text.parse().unwrap()
    } ).collect::<Vec<_>>();
    let t = data.remove(0);
    for test in 1..=t {
        let x = 0;

        let result = format!("Case #{}: {}\n", test, risolvi(&mut data));
        output.write(result.as_bytes()).unwrap();
    }
}
