use crate::dom::convert_dom::convert_node;
use crate::dom::structures::Node;
use html5ever::driver::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

fn parse(html: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default()).one(html)
}

pub fn parse_html(html: &str) -> Option<Node> {
    let dom = parse(html);

    for child in dom.document.children.borrow().iter() {
        if let Some(node) = convert_node(child) {
            return Some(node);
        }
    }

    None
}
