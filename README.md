# Huffman Coding

Disclaimer: most of the code was generated using ChatGPT.

One really cool thing about this solution is the `Ord` trait for `Node`:

```rust
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
```

Because it uses `self.frequency.cmp(&other.frequency)`, we would have a Max Heap and this wouldn't work. However, when also comparing the letter we ensure it works.
