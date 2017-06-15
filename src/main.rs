#![feature(box_patterns, box_syntax)]

use std::io;
use std::collections::HashMap;

pub mod huffman;
use huffman::Node;

/// Huffman algorithm as defined here: https://www.siggraph.org/education/materials/HyperGraph/video/mpeg/mpegfaq/huffman_tutorial.html


fn read_input() -> String {

    /// Read user input and remove white chars.

    println!("Type your text to encode.");

    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            println!("Your text: {}", buffer);
        }
        Err(error) => {
            println!("Error! {}", error);
        }
    }
    return buffer.trim().to_string();
}


fn encode_char(letter: &str, dictionary: &HashMap<String, Vec<i32>>) -> String {

    /// Replace a letter with the corresponding list of numbers.

    let codes = dictionary.get(letter).unwrap();
    let converted: Vec<String> = codes.iter().map(|a| a.to_string()).collect();

    return converted.join("");;
}


fn main() {

    /// Some arbitrary letters and the probability of their occurence in text.

    let available_letters = ["A", "B", "C", "D"];

    let mut nodes = vec![
        Node {symbol: "A", prob: 0.4, left: None, right: None},
        Node {symbol: "B", prob: 0.3, left: None, right: None},
        Node {symbol: "C", prob: 0.2, left: None, right: None},
        Node {symbol: "D", prob: 0.1, left: None, right: None},
    ];

    while nodes.len() > 1 {

        Node::create_branch(&mut nodes);

    }

    let root: Node = nodes.pop().unwrap();
    let result_list = root.generate_codes();
    let dictionary = root.build_dictionary(result_list);

    let to_encode_text = read_input();
    let text_vec = to_encode_text.split("")
                                 .filter_map(|c| if available_letters.contains(&c) { Some(c) } else { None })
                                 .collect::<Vec<&str>>();
    let result: Vec<String> = text_vec.iter()
                                      .map(|elem| encode_char(elem, &dictionary))
                                      .collect();
    println!("Result: {:?}", result);

}
