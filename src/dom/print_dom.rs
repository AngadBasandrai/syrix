use super::structures::{Node, ElementNode, TextNode};

fn print_node(node: &Node, depth: usize){
    match node {
        Node::Element(elem) => element_handler(elem, depth),
        Node::Text(text) => text_handler(text, depth),
    }
}

fn element_handler(elem: &ElementNode, depth: usize){
    println!("{}<{}>", "\t".repeat(depth), elem.tag());
    for child in elem.children() {
        print_node(child, depth + 1);
    }
    println!("{}</{}>", "\t".repeat(depth), elem.tag());
}

fn text_handler(text: &TextNode, depth: usize){
    println!("{}{}", "\t".repeat(depth), text.content());
}

pub fn print_tree(root: &Node){
    print_node(root, 0);
}