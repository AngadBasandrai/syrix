use std::collections::HashMap;

pub enum Node {
    Element(ElementNode),
    Text(TextNode),
}

pub struct ElementNode {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

impl ElementNode {
    pub fn new(tag: impl Into<String>, attributes: HashMap<String, String>, children: Vec<Node>,) -> Self {
        Self { tag: tag.into(), attributes, children }
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

pub struct TextNode {
    content: String
}

impl TextNode {

    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}