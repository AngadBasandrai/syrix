use std::{cell::RefCell, collections::HashMap};

use super::structures::{ElementNode, Node, TextNode};
use html5ever::{Attribute, QualName};
use markup5ever_rcdom::{Node as HtmlNode, NodeData};

pub fn convert_node(html_node: &HtmlNode) -> Option<Node> {
    match &html_node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point,
        } => Some(element_converter(name, attrs, html_node)),
        NodeData::Comment { contents } => None,
        NodeData::Doctype {
            name,
            public_id,
            system_id,
        } => None,
        NodeData::Document => None,
        NodeData::Text { contents } => {
            Some(Node::Text(TextNode::new(contents.borrow().to_string())))
        }
        NodeData::ProcessingInstruction { target, contents } => None,
    }
}

fn element_converter(
    name: &QualName,
    attrs: &RefCell<Vec<Attribute>>,
    html_node: &HtmlNode,
) -> Node {
    let mut attrs_map: HashMap<String, String> = HashMap::new();
    for attr in attrs.borrow().iter() {
        attrs_map.insert(attr.name.local.to_string(), attr.value.to_string());
    }

    let mut children = Vec::new();
    for child in html_node.children.borrow().iter() {
        if let Some(node) = convert_node(child) {
            children.push(node);
        }
    }

    Node::Element(ElementNode::new(
        name.local.to_string(),
        attrs_map,
        children,
    ))
}
