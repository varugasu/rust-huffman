use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    frequency: usize,
    character: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => match (&self.character, &other.character) {
                (Some(a), Some(b)) => a.cmp(b), // Compare characters if both are Some
                (Some(_), None) => Ordering::Less, // Always prioritize nodes with characters over None
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal, // If both are None, they are equal in priority
            },
            other_order => other_order,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_frequency_table(data: &str) -> HashMap<char, usize> {
    let mut frequency_table = HashMap::new();
    for ch in data.chars() {
        *frequency_table.entry(ch).or_insert(0) += 1;
    }
    frequency_table
}

pub fn build_huffman_tree(frequency_table: HashMap<char, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();
    for (ch, freq) in frequency_table {
        heap.push(Node {
            frequency: freq,
            character: Some(ch),
            left: None,
            right: None,
        });
    }

    println!("{:#?}", heap);

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let new_node = Node {
            frequency: left.frequency + right.frequency,
            character: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(new_node);
    }

    heap.pop().map(Box::new)
}

pub fn build_huffman_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(ch) = node.character {
        codes.insert(ch, prefix);
    } else {
        if let Some(ref left) = node.left {
            build_huffman_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            build_huffman_codes(right, format!("{}1", prefix), codes);
        }
    }
}

pub fn huffman_encode(data: &str) -> String {
    let frequency_table = build_frequency_table(data);
    let huffman_tree = build_huffman_tree(frequency_table.clone()).unwrap();

    let mut huffman_codes = HashMap::new();
    build_huffman_codes(&huffman_tree, String::new(), &mut huffman_codes);

    data.chars()
        .map(|ch| huffman_codes.get(&ch).unwrap().clone())
        .collect()
}

pub fn huffman_decode(encoded_data: &str, root: &Node) -> String {
    let mut decoded = String::new();
    let mut current_node = root;

    for bit in encoded_data.chars() {
        current_node = if bit == '0' {
            current_node.left.as_ref().unwrap()
        } else {
            current_node.right.as_ref().unwrap()
        };

        if let Some(ch) = current_node.character {
            decoded.push(ch);
            current_node = root;
        }
    }

    decoded
}
