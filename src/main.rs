#![feature(box_patterns, box_syntax)]

use std::io;
use std::collections::HashMap;

pub mod huffman;
use huffman::{Node, get_two_lowest};


fn read_input() -> String {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            println!("Your text: {}", buffer);
        }
        Err(error) => {
            println!("Error! {}", error);
        }
    }
    return buffer.trim().to_string();
}


fn encode_char(letter: &str, dictionary: &HashMap<String, Vec<i32>>) -> String {

    let codes = dictionary.get(letter).unwrap();
    let converted: Vec<String> = codes.iter().map(|a| a.to_string()).collect();

    return converted.join("");;
}


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

    let to_encode_text = read_input();
    let text_vec = to_encode_text.split("").filter_map(|c| if ["A", "B", "C", "D"].contains(&c) { Some(c) } else { None }).collect::<Vec<&str>>();
    let result: Vec<String> = text_vec.iter().map(|elem| encode_char(elem, &dictionary)).collect();
    println!("result: {:?}", result);

}
