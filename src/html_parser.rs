use crate::dom;
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Read the current character
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Check if proceeding characters start with string s
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if EOF
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return current character and advance position
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // Consume characters until test fails
    fn consume_while_char<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    // Consume characters until comment end
    // TODO: Could be improved (merge into one func with the prior one)
    fn consume_while_str(&mut self) -> String {
        let mut result = String::new();
        while !self.eof() && !self.input[self.pos..(self.pos + 3)].eq("-->") {
            result.push(self.consume_char());
        }
        return result;
    }

    // Consume and discard whitespaces
    fn consume_whitespace(&mut self) {
        self.consume_while_char(char::is_whitespace);
    }

    // Parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while_char(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // Parse a single node
    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<!--") {
            self.parse_comment()
        } else {
            match self.next_char() {
                '<' => self.parse_element(),
                _ => self.parse_text(),
            }
        }
    }

    // Parse a comment
    fn parse_comment(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '!');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        let comment = self.consume_while_str();
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '>');
        return dom::comment(comment);
    }

    // Parse a text node
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while_char(|c| c != '<'))
    }

    // Parse an element, including opening tag, contents, and closing tag
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::element(tag_name, attrs, children);
    }

    // Parse a single name=value pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse a quoted value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while_char(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // Parse a list of name=value pairs separated by whitespace
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // Parse a sequence of sibling nodes
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

// Parse HTML document and return the root element
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it, otherwise create one
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::element("html".to_string(), HashMap::new(), nodes)
    }
}
