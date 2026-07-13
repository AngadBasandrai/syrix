use super::structures::{ElementNode, Node, TextNode};

fn print_node(node: &Node, depth: usize) {
    if let Some(elem) = node.as_element() {
        element_handler(elem, depth);
    } else if let Some(text) = node.as_text() {
        text_handler(text, depth);
    }
}

fn element_handler(elem: &ElementNode, depth: usize) {
    print!("{}<{}", "\t".repeat(depth), elem.tag());
    for (key, value) in elem.attributes() {
        print!(" {}=\"{}\"", key, value);
    }
    println!(">");
    for child in elem.children() {
        print_node(child, depth + 1);
    }
    println!("{}</{}>", "\t".repeat(depth), elem.tag());
}

fn text_handler(text: &TextNode, depth: usize) {
    println!("{}{}", "\t".repeat(depth), text.content());
}

pub fn print_tree(root: &Node) {
    print_node(root, 0);
}
