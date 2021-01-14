use test_profiler;
use profiler_macro::profile_func;
use std::{
    path::Path,
    fs::File,
    io::prelude::*,
};

#[profile_func]
fn factorial(x: u128) -> u128 {
    match x {
        0..=1 => 1,
        _ => x * factorial(x - 1),
    }
}

fn main() {
    test_profiler::init_profiler();
    println!("{}", factorial(10));

    let path = Path::new(r"C:\Users\raguc\TheCode\json_tracing.json");
    let mut file = File::create(&path).unwrap();
    file.write_all(test_profiler::get_trace_json().as_bytes()).unwrap();
}
