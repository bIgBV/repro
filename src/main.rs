use crypto::{digest::Digest, sha1::Sha1};

fn main() {
    let content = String::from("Some bytes to be crushed").into_bytes();

    let mut hasher = Sha1::new();
    hasher.input(&content);

    let mut result: Vec<u8> = vec![];
    result.reserve(hasher.output_bytes());
    println!("Capacity: {}", result.capacity());

    hasher.result(&mut result);
}
