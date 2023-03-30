//! Author: sanyuan0704
//!
//! This plugin is used to construct the code block in mdx.

use hast;

fn transform_pre_code_element(node: &mut hast::Node) {
  // find the  <pre><code className="language-jsx">
  // and then transform it
  if let hast::Node::Element(node) = node {
    if node.tag_name == "pre" {
      // Check the `className` property for the `code` node
      // If the `className` is `code`, we stop the transformation
      if let Some((_, hast::PropertyValue::SpaceSeparated(class_names))) =
        node.properties.iter().find(|(key, _)| key == "className")
      {
        if class_names.contains(&"code".into()) {
          return;
        }
      }

      let mut code_node = None;
      if let hast::Node::Element(child) = node.children.first().unwrap() {
        if child.tag_name == "code" {
          code_node = Some(child);
        }
      }
      if let Some(code_node) = code_node {
        // get the className and meta of the code node, from its properties
        let mut meta = String::new();
        let mut title = None;
        for (key, value) in &code_node.properties {
          if key == "meta" {
            if let hast::PropertyValue::SpaceSeparated(values) = value {
              if let Some(value) = values.first() {
                meta = value.to_string();
              }
            }
          }
        }
        for part in meta.split(',') {
          let part = part.trim();
          if part.starts_with("title=") {
            title = Some(part[6..].trim_matches('"'));
          }
        }
        let title_node = if let Some(title) = title {
          Some(hast::Node::Element(hast::Element {
            tag_name: "div".into(),
            properties: vec![(
              "className".into(),
              hast::PropertyValue::SpaceSeparated(vec!["modern-code-title".into()]),
            )],
            children: vec![hast::Node::Text(hast::Text {
              value: title.to_string(),
              position: None,
            })],
            position: None,
          }))
        } else {
          None
        };

        let content_node = hast::Node::Element(hast::Element {
          tag_name: "div".into(),
          properties: vec![(
            "className".into(),
            hast::PropertyValue::SpaceSeparated(vec!["modern-code-content".into()]),
          )],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "button".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["copy".into()]),
              )],
              children: vec![],
              position: None,
            }),
            hast::Node::Element(hast::Element {
              tag_name: "pre".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["code".into()]),
              )],
              children: vec![hast::Node::Element(code_node.clone())],
              position: None,
            }),
          ],
          position: None,
        });

        node.tag_name = "div".into();

        node.properties = vec![(
          "className".into(),
          hast::PropertyValue::SpaceSeparated(vec!["language-".into()]),
        )];
        node.children = vec![
          title_node.unwrap_or(hast::Node::Text(hast::Text {
            value: "".into(),
            position: None,
          })),
          content_node,
        ]
      }
      // if the className is "language-jsx", we regard the lang as "jsx"
      // and parse the title from the meta
    }
  }
}

fn mdx_plugin_code_block_impl(node: &mut hast::Node) {
  transform_pre_code_element(node);

  if let Some(children) = node.children_mut() {
    for child in children {
      mdx_plugin_code_block_impl(child);
    }
  }
}

pub fn mdx_plugin_code_block(root: &mut hast::Node) {
  // Traverse all the hast node, and find the code element within pre, and then find the className of the code element
  // If the className is "language-jsx", we regard the lang as "jsx"

  // for example:
  // <pre>
  //   <code className="language-jsx">
  //     <p>hello world</p>
  //   </code>
  // </pre>
  // Will be transformed to:
  // <div className="language-jsx">
  //   <div className="modern-code-title">title</div>
  //   <div className="modern-code-content">
  //       <button className="copy"></button>
  //       <pre>
  //           <code className="language-jsx">
  //             <p>hello world</p>
  //           </code>
  //       </pre>
  //   </div>
  // </div>
  mdx_plugin_code_block_impl(root);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_transform_pre_code_element() {
    // Create a sample hast node
    let mut root = hast::Node::Element(hast::Element {
      tag_name: "pre".into(),
      properties: vec![(
        "className".into(),
        hast::PropertyValue::SpaceSeparated(vec!["language-rust".into()]),
      )],
      children: vec![hast::Node::Element(hast::Element {
        tag_name: "code".into(),
        properties: vec![(
          "meta".into(),
          hast::PropertyValue::SpaceSeparated(vec!["title=\"My Rust Code\"".into()]),
        )],
        children: vec![hast::Node::Text(hast::Text {
          value: "fn main() {\n   println!(\"Hello, world!\");\n}".into(),
          position: None,
        })],
        position: None,
      })],
      position: None,
    });

    mdx_plugin_code_block_impl(&mut root);
    // Check if the transformation was successful
    assert_eq!(
      root,
      hast::Node::Element(hast::Element {
        tag_name: "div".into(),
        properties: vec![(
          "className".into(),
          hast::PropertyValue::SpaceSeparated(vec!["language-".into()]),
        )],
        children: vec![
          hast::Node::Element(hast::Element {
            tag_name: "div".into(),
            properties: vec![(
              "className".into(),
              hast::PropertyValue::SpaceSeparated(vec!["modern-code-title".into()]),
            )],
            children: vec![hast::Node::Text(hast::Text {
              value: "My Rust Code".into(),
              position: None,
            })],
            position: None,
          }),
          hast::Node::Element(hast::Element {
            tag_name: "div".into(),
            properties: vec![(
              "className".into(),
              hast::PropertyValue::SpaceSeparated(vec!["modern-code-content".into()]),
            )],
            children: vec![
              hast::Node::Element(hast::Element {
                tag_name: "button".into(),
                properties: vec![(
                  "className".into(),
                  hast::PropertyValue::SpaceSeparated(vec!["copy".into()]),
                )],
                children: vec![],
                position: None,
              }),
              hast::Node::Element(hast::Element {
                tag_name: "pre".into(),
                properties: vec![(
                  "className".into(),
                  hast::PropertyValue::SpaceSeparated(vec!["code".into()]),
                )],
                children: vec![hast::Node::Element(hast::Element {
                  tag_name: "code".into(),
                  properties: vec![(
                    "meta".into(),
                    hast::PropertyValue::SpaceSeparated(vec!["title=\"My Rust Code\"".into()]),
                  )],
                  children: vec![hast::Node::Text(hast::Text {
                    value: "fn main() {\n   println!(\"Hello, world!\");\n}".into(),
                    position: None,
                  }),],
                  position: None,
                }),],
                position: None,
              }),
            ],
            position: None,
          })
        ],
        position: None,
      })
    );
  }
}
