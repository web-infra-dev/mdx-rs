//! Author: sanyuan0704
//!
//! This plugin is used to parse the container in markdown.
use hast;

// parse title from `title="xxx"` or `title=xxx` or `title='xxx'`
pub fn parse_title_from_meta(title_meta: &str) -> String {
  let mut title = title_meta;
  let quote = title_meta.chars().nth(6).unwrap();
  if quote != '"' && quote != '\'' {
    // ignore the last char, bacause it is "}"
    let last_index = title.rfind("}").unwrap_or(title.len());
    title = &title[6..last_index];
  } else {
    title = &title[7..];
    // find the last index of quote
    let last_index = title.rfind(quote).unwrap_or(title.len());
    title = &title[..last_index];
  }
  title.to_string()
}

pub fn parse_container_meta(meta: &str) -> (String, String) {
  // 1. parse the content before \n
  // 2. parse the type after `:::`, such as tip, warning, etc. The type properly has a space before `:::`.
  // 3. parse the title, such as `:::tip title` or `:::tip title="title"`
  let mut lines = meta.lines();

  let first_line = lines.next().unwrap_or("");
  let mut type_and_title = first_line
    .splitn(2, ":::")
    .skip(1)
    .next()
    .unwrap_or("")
    .splitn(2, |c| c == ' ' || c == '{');
  // Parse the type and title individually. Such as :::tip title="title" -> ("tip", "title="title"}")
  let container_type = type_and_title.next().unwrap_or("").trim();
  // Get the content before \n and trim the space.
  let mut title = type_and_title.next().unwrap_or("").to_string();

  // The title is properly `title="title"` or `title='title'`, we need to parse this case.
  if title.starts_with("title=") {
    title = parse_title_from_meta(&title);
  }
  (container_type.into(), title.trim().to_string())
}

fn create_new_container_node(
  container_type: &str,
  container_title: &str,
  container_content: &Vec<hast::Node>,
) -> hast::Node {
  // if the container title is empty, we use the container type and use camel case.
  let title = if container_title.is_empty() {
    let mut title = container_type.to_string();
    title.make_ascii_uppercase();
    title
  } else {
    container_title.to_string()
  };

  let container_title_node = hast::Element {
    tag_name: "div".into(),
    properties: vec![(
      "className".into(),
      hast::PropertyValue::SpaceSeparated(vec!["modern-directive-title".into()]),
    )],
    children: vec![hast::Node::Text(hast::Text {
      value: title,
      position: None,
    })],
    position: None,
  };
  let container_content_node = hast::Element {
    tag_name: "div".into(),
    properties: vec![(
      "className".into(),
      hast::PropertyValue::SpaceSeparated(vec!["modern-directive-content".into()]),
    )],
    children: container_content.clone(),
    position: None,
  };
  hast::Node::Element(hast::Element {
    tag_name: "div".into(),
    properties: vec![(
      "className".into(),
      hast::PropertyValue::SpaceSeparated(vec!["modern-directive".into(), container_type.into()]),
    )],
    children: vec![
      hast::Node::Element(container_title_node),
      hast::Node::Element(container_content_node),
    ],
    position: None,
  })
}

fn traverse_children(root: &mut hast::Root) {
  let mut container_type = String::new();
  let mut container_title = String::new();
  let mut container_content = vec![];
  let mut container_content_start = false;
  let mut container_content_end = false;
  let mut container_content_start_index = 0;
  let mut container_content_end_index = 0;
  let mut index = 0;
  while index < root.children.len() {
    let child = &root.children[index];
    if let hast::Node::Element(element) = child {
      // Meet the start of the container
      if element.tag_name == "p" && !container_content_start {
        if let Some(hast::Node::Text(text)) = element.children.first() {
          if text.value.starts_with(":::") {
            (container_type, container_title) = parse_container_meta(&text.value);
            // If the second element is MdxExpression, we parse the value and reassign the container_title
            if let Some(hast::Node::MdxExpression(expression)) = element.children.get(1) {
              container_title = parse_title_from_meta(&expression.value);
            }
            container_content_start = true;
            container_content_start_index = index;
            // :::tip\nThis is a tip
            // We should record the `This is a tip`
            for line in text.value.lines().skip(1) {
              if line.ends_with(":::") {
                container_content_end = true;
                container_content_end_index = index;
                break;
              }
              container_content.push(hast::Node::Text(hast::Text {
                value: line.into(),
                position: None,
              }));
            }
          }
        }
      }

      // Collect the container content in current p tag
      if container_content_start && !container_content_end && element.children.len() > 0 {
        if element.tag_name == "p" {
          let mut fragments = vec![];
          for (i, child) in element.children.iter().enumerate() {
            // Skip the meta string and stop when we meet the end of container
            if i == 0 && index == container_content_start_index {
              continue;
            }

            if i == 1 && index == container_content_start_index {
              if let hast::Node::MdxExpression(expression) = child {
                if expression.value.starts_with("title=") {
                  continue;
                }
              }
            }

            if let hast::Node::Text(text) = child {
              if text.value.ends_with(":::") {
                let extra_text = text.value.split(":::").next().unwrap_or("");
                if !extra_text.is_empty() {
                  fragments.push(hast::Node::Text(hast::Text {
                    value: extra_text.into(),
                    position: None,
                  }));
                }
                container_content_end = true;
                container_content_end_index = index;
                break;
              }
            }
            fragments.push(child.clone());
          }
          if fragments.len() > 0 {
            if index == container_content_start_index {
              container_content.extend(fragments);
            } else {
              container_content.push(hast::Node::Element(hast::Element {
                tag_name: "p".into(),
                properties: element.properties.clone(),
                children: fragments,
                position: None,
              }));
            }
          }
        } else {
          container_content.push(child.clone());
        }
      }

      // Meet the end of the container
      if container_content_end {
        // We should remove the container content from the root children
        // And add the container element to the root children
        let new_container_children = create_new_container_node(
          container_type.as_str(),
          container_title.as_str(),
          &container_content,
        );

        root
          .children
          .drain(container_content_start_index..=container_content_end_index);

        root
          .children
          .insert(container_content_start_index, new_container_children);

        container_title = String::new();
        container_content = vec![];
        container_content_start = false;
        container_content_end = false;
        index = container_content_start_index;
      }
      index += 1;
      continue;
    }

    index += 1;

    if container_content_start && !container_content_end {
      // Exclude the MdxExpression、MdxjsEsm Node
      if let hast::Node::MdxJsxElement(_) | hast::Node::MdxjsEsm(_) | hast::Node::MdxExpression(_) =
        child
      {
        continue;
      }
      container_content.push(child.clone());
    }
  }
}

pub fn mdx_plugin_container(root: &mut hast::Node) {
  // Traverse children, get all p tags, check if they start with :::
  // If it is, it is regarded as container syntax, and the content from the beginning of ::: to the end of a certain ::: is regarded as a container
  // The element of this container is a div element, className is "modern-container"
  // for example:
  // :::tip
  // this is a tip
  // :::
  // Will be transformed to:
  // <div class="modern-container">
  //   <div class="modern-container-title">tip</div>
  //   <div class="modern-container-content">
  //     <p>This is a tip</p>
  //   </div>
  // </div>
  if let hast::Node::Root(root) = root {
    traverse_children(root);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_title_from_meta() {
    assert_eq!(parse_title_from_meta("title=\"Note\""), "Note");
    assert_eq!(parse_title_from_meta("title=\'Note\'"), "Note");
    assert_eq!(parse_title_from_meta("title=Note"), "Note");
  }

  #[test]
  fn test_parse_container_meta() {
    assert_eq!(
      parse_container_meta(":::tip Note"),
      ("tip".into(), "Note".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=\"Note\"}"),
      ("note".into(), "Note".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=\'Note\'}"),
      ("note".into(), "Note".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=Note}"),
      ("note".into(), "Note".into()),
    );
  }

  #[test]
  fn test_parse_container_meta_with_empty_title() {
    assert_eq!(parse_container_meta(":::tip"), ("tip".into(), "".into()),);

    assert_eq!(
      parse_container_meta(":::note{title=\"\"}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=\'\'}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=}"),
      ("note".into(), "".into()),
    );
  }

  #[test]
  fn test_parse_container_meta_with_empty_type() {
    assert_eq!(parse_container_meta(":::"), ("".into(), "".into()),);

    assert_eq!(
      parse_container_meta(":::note{title=\"\"}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=\'\'}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=}"),
      ("note".into(), "".into()),
    );
  }

  #[test]
  fn test_parse_container_meta_with_empty_type_and_title() {
    assert_eq!(parse_container_meta(":::"), ("".into(), "".into()),);

    assert_eq!(
      parse_container_meta(":::note{title=\"\"}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=\'\'}"),
      ("note".into(), "".into()),
    );

    assert_eq!(
      parse_container_meta(":::note{title=}"),
      ("note".into(), "".into()),
    );
  }

  #[test]
  fn test_container_plugin_with_normal_title() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::tip Note".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: "This is a tip".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::".into(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    });

    mdx_plugin_container(&mut root);

    assert_eq!(
      root,
      hast::Node::Root(hast::Root {
        children: vec![hast::Node::Element(hast::Element {
          tag_name: "div".into(),
          properties: vec![(
            "className".into(),
            hast::PropertyValue::SpaceSeparated(vec!["modern-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-title".into()])
              )],
              children: vec![hast::Node::Text(hast::Text {
                value: "Note".into(),
                position: None,
              })],
              position: None,
            }),
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-content".into()])
              )],
              children: vec![hast::Node::Element(hast::Element {
                tag_name: "p".into(),
                properties: vec![],
                children: vec![hast::Node::Text(hast::Text {
                  value: "This is a tip".into(),
                  position: None,
                })],
                position: None,
              })],
              position: None,
            })
          ],
          position: None,
        })],
        position: None,
      })
    );
  }

  #[test]
  fn test_container_plugin_with_empty_title() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::tip".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: "This is a tip".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::".into(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    });

    mdx_plugin_container(&mut root);

    assert_eq!(
      root,
      hast::Node::Root(hast::Root {
        children: vec![hast::Node::Element(hast::Element {
          tag_name: "div".into(),
          properties: vec![(
            "className".into(),
            hast::PropertyValue::SpaceSeparated(vec!["modern-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-title".into()])
              )],
              children: vec![hast::Node::Text(hast::Text {
                value: "TIP".into(),
                position: None,
              })],
              position: None,
            }),
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-content".into()])
              )],
              children: vec![hast::Node::Element(hast::Element {
                tag_name: "p".into(),
                properties: vec![],
                children: vec![hast::Node::Text(hast::Text {
                  value: "This is a tip".into(),
                  position: None,
                })],
                position: None,
              })],
              position: None,
            })
          ],
          position: None,
        })],
        position: None,
      })
    );
  }

  #[test]
  fn test_container_plugin_with_title_assign() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::tip{title=\"Note\"}".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: "This is a tip".into(),
            position: None,
          })],
          position: None,
        }),
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::".into(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    });

    mdx_plugin_container(&mut root);

    assert_eq!(
      root,
      hast::Node::Root(hast::Root {
        children: vec![hast::Node::Element(hast::Element {
          tag_name: "div".into(),
          properties: vec![(
            "className".into(),
            hast::PropertyValue::SpaceSeparated(vec!["modern-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-title".into()])
              )],
              children: vec![hast::Node::Text(hast::Text {
                value: "Note".into(),
                position: None,
              })],
              position: None,
            }),
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["modern-directive-content".into()])
              )],
              children: vec![hast::Node::Element(hast::Element {
                tag_name: "p".into(),
                properties: vec![],
                children: vec![hast::Node::Text(hast::Text {
                  value: "This is a tip".into(),
                  position: None,
                })],
                position: None,
              })],
              position: None,
            })
          ],
          position: None,
        })],
        position: None,
      })
    );
  }
}
