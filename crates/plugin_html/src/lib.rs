//! Author: sanyuan0704
//!
//! This plugin is used to serialize hast to html.

use hast::Node;

fn display_property_value(value: &hast::PropertyValue) -> String {
  match value {
    hast::PropertyValue::String(value) => value.clone(),
    hast::PropertyValue::Boolean(value) => value.to_string(),
    _ => "".to_string(),
  }
}

pub fn mdx_plugin_html_impl(node: &Node) -> String {
  match node {
    Node::Element(element) => {
      if element.tag_name == "script" || element.tag_name == "style" {
        return "".to_string();
      }
      let mut html = String::new();
      html.push_str(&format!("<{}", element.tag_name));
      for (key, value) in &element.properties {
        // skip className
        if key == "className" {
          continue;
        }
        html.push_str(&format!(" {}=\"{}\"", key, display_property_value(value)));
      }
      html.push_str(">");
      for child in &element.children {
        html.push_str(&mdx_plugin_html_impl(child));
      }
      html.push_str(&format!("</{}>", element.tag_name));
      html
    }
    Node::Text(text) => text.value.clone(),
    Node::Comment(_) => "".to_string(),
    Node::Root(root) => {
      let mut html = String::new();
      for child in &root.children {
        html.push_str(&mdx_plugin_html_impl(child));
      }
      html
    }
    _ => "".to_string(),
  }
}

pub fn mdx_plugin_html(node: &Node) -> String {
  mdx_plugin_html_impl(node)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialize_hast_to_html() {
    let text = Node::Text(hast::Text {
      value: "hello world".to_string(),
      position: None,
    });

    let element = Node::Element(hast::Element {
      tag_name: "div".to_string(),
      properties: vec![(
        "a".to_string(),
        hast::PropertyValue::String("1".to_string()),
      )],
      children: vec![text],
      position: None,
    });
    let root = Node::Root(hast::Root {
      children: vec![element],
      position: None,
    });

    let html = mdx_plugin_html(&root);

    assert_eq!(html, "<div a=\"1\">hello world</div>");
  }
}
