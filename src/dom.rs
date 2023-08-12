use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    _children: Vec<Node>,
    _node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug)]
pub struct ElementData {
    _tag_name: String,
    _attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        _children: Vec::new(),
        _node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        _children: children,
        _node_type: NodeType::Element(ElementData {
            _tag_name: name,
            _attributes: attrs,
        }),
    }
}

pub fn comment(data: String) -> Node {
    Node {
        _children: Vec::new(),
        _node_type: NodeType::Comment(data),
    }
}
