# Huffman encoding
Rust implementation of Huffman encoding algorithm inspired by [siggraph.org recipe](https://www.siggraph.org/education/materials/HyperGraph/video/mpeg/mpegfaq/huffman_tutorial.html)

This proof-of-concept version supports 4 letters: ABCD with fixed probabilities:

```rust
Node {symbol: "A", prob: 0.4, left: None, right: None},
Node {symbol: "B", prob: 0.3, left: None, right: None},
Node {symbol: "C", prob: 0.2, left: None, right: None},
Node {symbol: "D", prob: 0.1, left: None, right: None},
```

```
[sireliah@sir huffman]$ ./huffman_coding
Type your text to encode.
ACCCCD
Your text: ACCCCD

Result: ["0", "101", "101", "101", "101", "100"]

```
