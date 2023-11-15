use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("expecteda readable string in stdin");
    println!("String, {}", input);

    let mut mem: [u32; 256] = vec![0; 256].try_into().unwrap();
    for &byte in &input.into_bytes() {
        let index = byte as usize;
        mem[index] += 1;
    }

    let res: u32 = mem.iter().filter(|&&i| i > 1).sum();
    if res == 0 {
        write!(&mut fptr, "Unique").ok();
    } else {
        write!(&mut fptr, "{:?} Duplicates", res).ok();
    }
    writeln!(&mut fptr).ok();
}
