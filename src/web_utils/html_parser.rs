
use yew::prelude::*;
use gloo::utils::document;
use web_sys::{Element, Node};

pub struct HtmlParser {}
impl HtmlParser {
    pub fn parse_string_vector_to_inner_html(&self, vect: &Vec<String>, class: &str) -> Vec<Html> {
        let nodes: Vec<Html> = vect
            .iter()
            .map(|item| {
                let p: Element = document().create_element("p").unwrap();
                p.set_inner_html(item);
                p.set_class_name(class);
                let node: Node = p.into(); // Element into Node
                Html::VRef(node.into()) // Node into VNode
            })
            .collect();
        nodes
    }
}
