use rust_huffman::{build_frequency_table, build_huffman_tree, huffman_decode, huffman_encode};

pub fn main() {
    let data = "abbcd";
    let frequency_table = build_frequency_table(data);
    println!("{:?}", frequency_table);
    let huffman_tree = build_huffman_tree(frequency_table).unwrap();

    println!("{:#?}", huffman_tree);

    let encoded = huffman_encode(data);
    println!("{:?}", encoded);

    let decoded = huffman_decode(&encoded, &huffman_tree);
    println!("{:?}", decoded);
}
