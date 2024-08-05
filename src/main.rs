use std::{io::Read, collections::HashMap};
use bit_vec::BitVec;
use lzw::encode;
use huffman_compress::CodeBuilder;

fn main() -> Result<(), std::io::Error> {
    let mut input = vec![];

    if let Some(filename) = std::env::args().nth(1) {
        input = std::fs::read(filename)?;
    }
    else {
        std::io::stdin().read_to_end(&mut input)?;
    }
    let uncompressed_len = input.len();
    println!("Uncompressed size:\t{:>16} bytes", uncompressed_len);

    let codes = encode(&input);
    let num_codes = codes.len();
    let max_code = codes.iter().max().unwrap();
    println!("LZW encoded size:\t{:>16} bytes ({} 64bit codes, biggest code is {})",
             num_codes*8,
             num_codes,
             max_code
    );

    let mut weights = HashMap::new();
    for code in &codes {
        *weights.entry(code).or_insert(1) += 1;
    }

    let (book, _) = CodeBuilder::from_iter(weights).finish();
    let mut buffer = BitVec::new();
    for code in &codes {
        book.encode(&mut buffer, code).ok();
    }

    let compressed_len = buffer.to_bytes().len();
    println!("Huffman coded size:\t{:>16} bytes", compressed_len);
    println!("{}% of original size", (compressed_len as f64 / uncompressed_len as f64) * 100.0);

    let symbols = book.len();
    println!("This is ignoring the size of the huffman tree which has {} leaves", symbols);

    Ok(())
}
