mod dom;
mod html_parser;

use std::fs;

fn print_dom(root: dom::Node) {
    println!("{:#?}", root);
}

fn main() {
    let contents = fs::read_to_string("index.html").expect("Unable to read file.");
    let parsed_root = html_parser::parse(contents);
    print_dom(parsed_root);
}
