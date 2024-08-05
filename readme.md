# Testing [LZW](https://github.com/Daniel-04/lzw) with [Huffman coding](https://github.com/niklasf/rust-huffman-compress)
- Compression ratio: Mediocre
- Performance: Abysmal
- Hotel: Trivago

Leaves much to be desired when compared with [this input](https://sun.aei.polsl.pl//~sdeor/index.php?page=silesia) against [these compressors](https://github.com/inikep/lzbench/blob/master/lzbench18_sorted.md):
```sh
$ cargo run --release silesia.tar 
Uncompressed size:             211957760 bytes
LZW encoded size:              227901016 bytes (28487627 64bit codes, biggest code is 28487880)
Huffman coded size:             79079452 bytes
37.30906195649548% of original size
This is ignoring the size of the huffman tree which has 11324446 leaves
```

## Usage examples
```sh
$ cargo run file.txt
```
```sh
$ cat file.txt | cargo run
```
