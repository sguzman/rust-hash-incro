extern crate md5;
extern crate rayon;

// Get first cmd arg
fn get_arg() -> String {
    std::env::args().take(1).collect::<String>()
}

// Load file content as byte array
fn load_file(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("Failed to read file")
}

fn main() {
    let arg = get_arg();
    let contents = load_file(&arg);
    println!("Loading file: {}", arg);
    println!("File content length: {}", contents.len());

    println!("Hello, world!");
}
