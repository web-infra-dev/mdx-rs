//! Author: sanyuan0704
//!
//! This plugin is used to add anchor to the header in link element.

use utils::extract_title_and_id;
use {hast, slugger::Slugger};

fn collect_title_in_hast(node: &mut hast::Element) -> (String, String) {
  let mut title = String::new();
  let mut id = String::new();
  let mut custom_id_expression_index = None;
  for (index, child) in &mut node.children.iter_mut().enumerate() {
    match child {
      // example: hello world {#custom-id}
      // Then we extract the `hello world` as title and `custom-id` as id
      // .md case:
      hast::Node::Text(text) => {
        let (title_part, id_part) = extract_title_and_id(&text.value);
        let title_str = title_part.trim_end();
        title.push_str(title_str);
        text.value = title_str.to_string();
        id = id_part;
      }
      // .mdx case:
      hast::Node::MdxExpression(expression) => {
        if expression.value.starts_with("#") {
          id.push_str(&expression.value[1..]);
          custom_id_expression_index = Some(index);
        }
      }
      hast::Node::Element(element) => {
        if element.tag_name == "code" {
          for child in &element.children {
            if let hast::Node::Text(text) = child {
              title.push_str(&text.value);
            }
          }
        }
      }

      _ => continue, // Continue if node is not Text or Code
    }
  }
  if let Some(index) = custom_id_expression_index {
    node.children.remove(index);
  }

  (title, id)
}

fn create_anchor_element(id: &str) -> hast::Element {
  hast::Element {
    tag_name: "a".to_string(),
    properties: vec![
      // Add the class name: `header-anchor`
      (
        "className".to_string(),
        hast::PropertyValue::SpaceSeparated(vec!["header-anchor".to_string()]),
      ),
      // Add the attribute: `aria-hidden="true"`
      (
        "aria-hidden".to_string(),
        hast::PropertyValue::String("true".to_string()),
      ),
      // Add the attribute: `href="#${id}"`
      (
        "href".to_string(),
        hast::PropertyValue::String(format!("#{}", id)),
      ),
    ],
    children: vec![
      // # is the content of the anchor element
      hast::Node::Text(hast::Text {
        value: "#".to_string(),
        position: None,
      }),
    ],
    position: None,
  }
}

// In this plugin, we do the following things:
// 1. add header anchor for every header element
// 2. add target="_blank" and rel="noopener noreferrer" for every external link element
pub fn mdx_plugin_header_anchor(node: &mut hast::Node) {
  let mut slugger: Slugger = Slugger::new();
  if let hast::Node::Root(root) = node {
    for child in &mut root.children {
      if let hast::Node::Element(element) = child {
        if let Some(h_tag) = element.tag_name.chars().nth(1).and_then(|c| c.to_digit(10)) {
          // h1 ~ h6
          if h_tag >= 1 && h_tag <= 6 {
            // get the text of the header element
            let (header_text, mut id) = collect_title_in_hast(element);
            if id.is_empty() {
              id = slugger.slug(&header_text, false);
            }
            let id_property = ("id".to_string(), hast::PropertyValue::String(id.clone()));
            // add the id attribute to the header element
            element.properties.push(id_property);

            // add the anchor element to the header element
            element
              .children
              .push(hast::Node::Element(create_anchor_element(&id)));
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use hast::Node;

  #[test]
  fn test_collect_title_in_hast() {
    let mut element = hast::Element {
      tag_name: "h1".to_string(),
      properties: vec![],
      children: vec![
        Node::Text(hast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        Node::Element(hast::Element {
          tag_name: "code".to_string(),
          properties: vec![],
          children: vec![Node::Text(hast::Text {
            value: "World".to_string(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    };

    assert_eq!(
      collect_title_in_hast(&mut element),
      ("HelloWorld".to_string(), "".to_string())
    );
  }

  #[test]
  fn test_create_anchor_element() {
    let element = create_anchor_element("hello-world");
    assert_eq!(element.tag_name, "a");
    assert_eq!(
      element.properties,
      vec![
        (
          "className".to_string(),
          hast::PropertyValue::SpaceSeparated(vec!["header-anchor".to_string()])
        ),
        (
          "aria-hidden".to_string(),
          hast::PropertyValue::String("true".to_string())
        ),
        (
          "href".to_string(),
          hast::PropertyValue::String("#hello-world".to_string())
        ),
      ]
    );
    assert_eq!(
      element.children,
      vec![Node::Text(hast::Text {
        value: "#".to_string(),
        position: None,
      })]
    );
  }

  #[test]
  fn test_mdx_plugin_header_anchor() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![
        Node::Element(hast::Element {
          tag_name: "h1".to_string(),
          properties: vec![],
          children: vec![Node::Text(hast::Text {
            value: "Hello World".to_string(),
            position: None,
          })],
          position: None,
        }),
        Node::Element(hast::Element {
          tag_name: "h2".to_string(),
          properties: vec![],
          children: vec![Node::Text(hast::Text {
            value: "Hello World".to_string(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    });

    mdx_plugin_header_anchor(&mut root);

    let children = match root {
      hast::Node::Root(root) => root.children,
      _ => panic!("root should be a Root node"),
    };

    assert_eq!(
      children,
      vec![
        Node::Element(hast::Element {
          tag_name: "h1".to_string(),
          properties: vec![(
            "id".to_string(),
            hast::PropertyValue::String("hello-world".to_string())
          ),],
          children: vec![
            Node::Text(hast::Text {
              value: "Hello World".to_string(),
              position: None,
            }),
            Node::Element(hast::Element {
              tag_name: "a".to_string(),
              properties: vec![
                (
                  "className".to_string(),
                  hast::PropertyValue::SpaceSeparated(vec!["header-anchor".to_string()])
                ),
                (
                  "aria-hidden".to_string(),
                  hast::PropertyValue::String("true".to_string())
                ),
                (
                  "href".to_string(),
                  hast::PropertyValue::String("#hello-world".to_string())
                ),
              ],
              children: vec![Node::Text(hast::Text {
                value: "#".to_string(),
                position: None,
              })],
              position: None,
            }),
          ],
          position: None,
        }),
        Node::Element(hast::Element {
          tag_name: "h2".to_string(),
          properties: vec![(
            "id".to_string(),
            hast::PropertyValue::String("hello-world-1".to_string())
          ),],
          children: vec![
            Node::Text(hast::Text {
              value: "Hello World".to_string(),
              position: None,
            }),
            Node::Element(hast::Element {
              tag_name: "a".to_string(),
              properties: vec![
                (
                  "className".to_string(),
                  hast::PropertyValue::SpaceSeparated(vec!["header-anchor".to_string()])
                ),
                (
                  "aria-hidden".to_string(),
                  hast::PropertyValue::String("true".to_string())
                ),
                (
                  "href".to_string(),
                  hast::PropertyValue::String("#hello-world-1".to_string())
                ),
              ],
              children: vec![Node::Text(hast::Text {
                value: "#".to_string(),
                position: None,
              })],
              position: None,
            }),
          ],
          position: None,
        }),
      ]
    );
  }
}
