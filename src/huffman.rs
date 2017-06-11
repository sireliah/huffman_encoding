
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<'z> {
    pub symbol: &'z str,
    pub prob: f32,
    /// Box reallocates the node from stack to heap. 
    /// Otherwise, the compiler will complain that "recursive type `Node` has infinite size"
    pub left: Option<Box<Node<'z>>>,
    pub right: Option<Box<Node<'z>>>,
}


impl<'z> Node<'z> {

    pub fn build_dictionary(&self, result_list: Vec<(&Node, Vec<i32>)>) -> HashMap<String, Vec<i32>> {

        /// Create dictionary based on nodes in the vector.
        /// TODO: create this using map()

        let mut dictionary = HashMap::new();
        for tuple in &result_list {
            if tuple.0.symbol != "*" {

                let codes = tuple.1.clone();
                let symbol = tuple.0.symbol.to_string().clone();
                dictionary.insert(symbol, codes);
            }
        }

        return dictionary;
    }

    pub fn generate_codes(&self) -> Vec<(&Node, Vec<i32>)> {

        /// Traverse the tree using a simple queue algorithm. 
        /// Store results in pair (Node, turned) where "turned" determines 
        /// if we descended left or right.

        let mut stack = Vec::<(&Node, Vec<i32>)>::new();
        let mut result = Vec::<(&Node, Vec<i32>)>::new();

        stack.push((self, vec![]));

        /// Loop until you reach all nodes that don't have left or right children.
        while !stack.is_empty() {

            let (node, codes) = stack.pop().unwrap();
            let copied_node = node.clone();
            let copied_codes = codes.clone();

            result.push((copied_node, copied_codes));

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

    pub fn create_branch(nodes: &mut Vec<Node>) {

        /// Create new branch whit two lowest nodes as (left and right) children.

        /// Sort by probability and pop two lowest.
        nodes.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());

        let first = match nodes.pop() { 
            Some(i) => i,
            None => return,
        };

        let second: Node = match nodes.pop() { 
            Some(i) => i,
            None => return,
        };

        /// Create new node and push it to the vector.
        let new_node = Node {
                symbol: "*",
                prob: first.prob + second.prob,
                left: Some(Box::new(first)),
                right: Some(Box::new(second)),
        };

        nodes.push(new_node);

    }
}

