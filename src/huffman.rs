#![feature(box_patterns, box_syntax)]

use std::collections::HashMap;

/// "z" is our lifetime parameter.
#[derive(Debug)]
struct Node<'z> {
    symbol: &'z str,
    prob: f32,
    /// Box reallocates the node from stack to heap. Otherwise, the compiler will complain that "recursive type `Node` has infinite size"
    left: Option<Box<Node<'z>>>,
    right: Option<Box<Node<'z>>>,
}

impl<'z> Node<'z> {

    //    fn new_node(symbol: &'z str, first: Node<'z>, second: Node<'z>) -> Node<'z> {
    //        Node {
    //            symbol: symbol,
    //            prob: first.prob + second.prob,
    //            left: Some(Box::new(first)),
    //            right: Some(Box::new(second)),
    //        }
    //    }

    fn build_dictionary(&self, result_list: Vec<(&Node, Vec<i32>)>) -> HashMap<String, Vec<i32>> {

        /// TODO: create this using map()
        let mut dictionary = HashMap::new();
        for tuple in &result_list {
            if tuple.0.symbol != "*" {
                //println!("{}, {:?}", tuple.0.symbol, tuple.1);

                let codes = tuple.1.clone();
                let symbol = tuple.0.symbol.to_string().clone();
                dictionary.insert(symbol, codes);
            }
        }

        return dictionary;
    }

    fn generate_codes(&self) -> Vec<(&Node, Vec<i32>)> {

        let mut stack = Vec::<(&Node, Vec<i32>)>::new();
        let mut result = Vec::<(&Node, Vec<i32>)>::new();

        // let mut code = "".as_string();
        stack.push((self, vec![]));

        while !stack.is_empty() {

            let (node, codes) = stack.pop().unwrap();
            let copied_node = node.clone();
            let copied_codes = codes.clone();

            result.push((copied_node, copied_codes));
            //println!("{}, {}", node.symbol, node.prob);
            //println!("{:?}", codes);

            if let Some(box ref nod) = node.right {
                let mut new_codes = codes.clone();
                new_codes.push(1);
                stack.push((nod, new_codes));
            }

            if let Some(box ref nod) = node.left {
                let mut new_codes = codes.clone();
                new_codes.push(0);
                stack.push((nod, new_codes));
            }

        }

        return result;
    }



}

fn get_two_lowest(nodes: &mut Vec<Node>) -> bool {
    /// First sort by probability.
    nodes.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());

    let first = match nodes.pop() { 
        Some(i) => i,
        None => return true,
    };

    let second: Node = match nodes.pop() { 
        Some(i) => i,
        None => return true,
    };

    //let new_node = Node::new_node("*", first, second);

    /// Create new node with lowest nodes as children.
    let new_node = Node {
            symbol: "*",
            prob: first.prob + second.prob,
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
    };

    nodes.push(new_node);

    return false;
}

// fn traverse(node: Node) {

//     let mut stack: Vec<&Node> = Vec::new();
//     let mut res: Vec<&Node> = Vec::new();

//     match node.left {
//         Some(left) => encode(left, "left"),
//         None => println!("nothin' on left found"),
//     }

//     match node.right {
//         Some(right) => encode(right, "right"),
//         None => println!("nothin' on right found"),
//    }
//}

fn main() {
    //let mut key_map = HashMap::new();
    let mut nodes = vec![
        Node {symbol: "A", prob: 0.4, left: None, right: None},
        Node {symbol: "B", prob: 0.3, left: None, right: None},
        Node {symbol: "C", prob: 0.2, left: None, right: None},
        Node {symbol: "D", prob: 0.1, left: None, right: None},
    ];

    //let mut stop = false;

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