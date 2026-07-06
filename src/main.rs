mod util;
mod dom;

use std::collections::HashMap;

use dom::print_dom::print_tree;
use dom::structures::{ElementNode, Node, TextNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let mut url = String::new();
    // std::io::stdin().read_line(&mut url).unwrap();
    // url = url.trim().to_string();

    // let html = util::fetch::fetch(&url)?;

    // let page = util::page::Page::new(url, html);

    // page.print_summary();

    let root = Node::Element(
        ElementNode::new(
            "body",
            HashMap::new(),
            vec![
                Node::Element(
                    ElementNode::new(
                        "h1",
                        HashMap::new(),
                        vec![
                            Node::Text(TextNode::new("Hello"))
                        ],
                    )
                ),
                Node::Element(
                    ElementNode::new(
                        "p",
                        HashMap::new(),
                        vec![
                            Node::Text(TextNode::new("Bello"))
                        ],
                    )
                ),
            ],
        ),
    );

    print_tree(&root);


    Ok(())

}
