#![feature(box_patterns, box_syntax)]

pub mod huffman;
use huffman::{Node, get_two_lowest};


fn main() {

    let mut nodes = vec![
        Node {symbol: "A", prob: 0.4, left: None, right: None},
        Node {symbol: "B", prob: 0.3, left: None, right: None},
        Node {symbol: "C", prob: 0.2, left: None, right: None},
        Node {symbol: "D", prob: 0.1, left: None, right: None},
    ];

    while nodes.len() > 1 {

        get_two_lowest(&mut nodes);

    }

    println!("Number of nodes: {}", nodes.len());

    for node in &nodes {
        println!("Iterating nodes: symbol: {}, prob: {}", node.symbol, node.prob);
    }

    let root: Node = nodes.pop().unwrap();
    let result = root.generate_codes();
    let dictionary = root.build_dictionary(result);
    println!("{:?}", dictionary);

}
