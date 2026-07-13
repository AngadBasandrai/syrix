use crate::dom::structures::{ElementNode, Node, TextNode};

pub fn render(node: &Node) {
    if let Some(elem) = node.as_element() {
        render_element(elem);
    } else if let Some(text) = node.as_text() {
        print!("{}", text.content());
    }
}

fn render_element(elem: &ElementNode) {
    match elem.tag() {
        "h1" => render_h1(elem),
        "h2" => render_h2(elem),
        "h3" => render_h3(elem),
        "h4" => render_h4(elem),
        "br" => render_br(elem),
        "b" | "strong" => render_b(elem),
        "i" | "em" => render_i(elem),
        "p" => render_p(elem),
        "a" => render_a(elem),
        "body" | "div" => render_default(elem),
        "head" => return,
        "script" => return,
        _ => render_default(elem),
    }
}

fn render_h1(elem: &ElementNode) {
    print!("# ");

    render_children(elem);

    println!();
}

fn render_h2(elem: &ElementNode) {
    print!("## ");

    render_children(elem);

    println!();
}

fn render_h3(elem: &ElementNode) {
    print!("### ");

    render_children(elem);

    println!();
}

fn render_h4(elem: &ElementNode) {
    print!("#### ");

    render_children(elem);

    println!();
}

fn render_br(elem: &ElementNode) {
    println!();
    render_children(elem);
}

fn render_b(elem: &ElementNode) {
    print!("**");

    render_children(elem);

    print!("**");
}

fn render_i(elem: &ElementNode) {
    print!("__");

    render_children(elem);

    print!("__");
}

fn render_p(elem: &ElementNode) {
    render_children(elem);
    println!();
}

fn render_a(elem: &ElementNode) {
    render_children(elem);
}

fn render_default(elem: &ElementNode) {
    render_children(elem);
}

fn render_children(elem: &ElementNode) {
    for child in elem.children() {
        render(child);
    }
}
