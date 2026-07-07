mod dom;
mod parser;
mod util;

use std::collections::HashMap;

use dom::print_dom::print_tree;
use dom::structures::{ElementNode, Node, TextNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();
    url = url.trim().to_string();

    let html = util::fetch::fetch(&url)?;

    let page = util::page::Page::new(url, html);

    page.print_summary();

    let dom = parser::parser::parse_html(page.html()).ok_or("Failed to parse HTML!")?;

    print_tree(&dom);

    Ok(())
}
