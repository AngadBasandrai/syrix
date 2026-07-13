use std::collections::HashMap;

pub enum Node {
    Element(ElementNode),
    Text(TextNode),
}

impl Node {
    pub fn is_element(&self) -> bool {
        matches!(self, Node::Element(_))
    }
    pub fn is_text(&self) -> bool {
        matches!(self, Node::Text(_))
    }
    pub fn as_element(&self) -> Option<&ElementNode> {
        match self {
            Node::Element(elem) => Some(elem),
            Node::Text(_) => None,
        }
    }
    pub fn as_text(&self) -> Option<&TextNode> {
        match self {
            Node::Element(_) => None,
            Node::Text(text) => Some(text),
        }
    }
}

pub struct ElementNode {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
}

impl ElementNode {
    pub fn new(
        tag: impl Into<String>,
        attributes: HashMap<String, String>,
        children: Vec<Node>,
    ) -> Self {
        Self {
            tag: tag.into(),
            attributes,
            children,
        }
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn attribute(&self, name: String) -> Option<&String> {
        self.attributes.get(&name)
    }
}

pub struct TextNode {
    content: String,
}

impl TextNode {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
