extern crate md5;
extern crate rayon;

use rayon::prelude::*;

// Get first cmd arg
fn get_arg() -> String {
    std::env::args().take(1).collect::<String>()
}

// Get md5 hash of a byte array
fn get_hash(bytes: &[u8]) -> String {
    format!("{:x}", md5::compute(bytes))
}

// Load file content as byte array
fn load_file(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("Failed to read file")
}

fn incr_hash(n: usize, data: Vec<u8>) -> Vec<String> {
    (0..n)
        .into_par_iter()
        .map(|limit| get_hash(&data[..limit]))
        .collect::<Vec<String>>()
}

fn sorted_hashes(hashes: Vec<String>) -> Vec<String> {
    let mut hashes = hashes.clone();
    hashes.par_sort();
    hashes
}

fn main() {
    println!("hi :)");
    let arg = get_arg();
    let contents = load_file(&arg);
    println!("Loading file: {}", arg);
    println!("File content length: {}", contents.len());

    let hashes = incr_hash(100, contents);
    let hashes = sorted_hashes(hashes);

    for hash in hashes {
        println!("Sorted \t{}", hash);
    }

    println!("bye :(");
}
