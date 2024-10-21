//! Author: sanyuan0704
//!
//! This plugin is used to parse the container in markdown.
#![allow(clippy::ptr_arg)]

// parse title from `title="xxx"` or `title=xxx` or `title='xxx'`
pub fn parse_title_from_meta(title_meta: &str) -> String {
  let mut title = title_meta;
  let quote = title_meta.chars().nth(6).unwrap();
  if quote != '"' && quote != '\'' {
    // ignore the last char, because it is "}"
    let last_index = title.rfind('}').unwrap_or(title.len());
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
    .split_once(":::")
    .map(|x| x.1)
    .unwrap_or("")
    .trim_start()
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

  let is_details = container_type == "details";
  let title_tag_name = if is_details { "summary" } else { "div" };
  let root_tag_name = if is_details { "details" } else { "div" };

  let container_title_node = hast::Element {
    tag_name: title_tag_name.into(),
    properties: vec![(
      "className".into(),
      hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()]),
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
      hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()]),
    )],
    children: container_content.clone(),
    position: None,
  };
  hast::Node::Element(hast::Element {
    tag_name: root_tag_name.into(),
    properties: vec![(
      "className".into(),
      hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), container_type.into()]),
    )],
    children: vec![
      hast::Node::Element(container_title_node),
      hast::Node::Element(container_content_node),
    ],
    position: None,
  })
}

fn wrap_node_with_paragraph(
  properties: &[(String, hast::PropertyValue)],
  children: &[hast::Node],
) -> hast::Node {
  let mut paragraph = hast::Element {
    tag_name: "p".into(),
    properties: properties.to_vec(),
    children: Vec::new(),
    position: None,
  };

  paragraph.children.extend_from_slice(children);

  hast::Node::Element(paragraph)
}

fn is_valid_container_type(container_type: &String) -> bool {
  let mut container_type = container_type.clone();
  container_type.make_ascii_lowercase();
  let valid_types = [
    "tip", "note", "warning", "caution", "danger", "info", "details",
  ];
  valid_types.contains(&container_type.as_str())
}

fn parse_github_alerts_container_meta(meta: &str) -> (String, String) {
  // GitHub Alert's verification is very strict.
  // space and breaks are not allowed, they must be a whole.
  // but can remove spaces or breaks at the beginning and end.
  let mut lines = meta.lines();

  let mut container_type = String::new();
  let mut remaining_data = String::new();

  let mut is_first_line = true;

  while let Some(line) = lines.next() {
    // clear breaks if no container_type
    if container_type.is_empty() && line.is_empty() {
      continue;
    }

    if container_type.is_empty() && is_first_line {
      is_first_line = false;

      let split_line = line.trim().split_once("]");

      container_type = split_line
        .unwrap_or(("", ""))
        .0
        .to_owned()
        .replace("[!", "");
      remaining_data = split_line.unwrap_or(("", "")).1.to_owned();

      if container_type.is_empty() {
        break;
      }

      continue;
    }

    if remaining_data.is_empty() {
      remaining_data = line.to_owned();
    } else {
      remaining_data = format!("{}\n{}", remaining_data, line);
    }
  }

  (container_type, remaining_data)
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
      if !container_content_start {
        // e.g. :::tip
        if element.tag_name == "p" {
          if let Some(hast::Node::Text(text)) = element.children.first() {
            if text.value.starts_with(":::") {
              (container_type, container_title) = parse_container_meta(&text.value);
              if !is_valid_container_type(&container_type) {
                index += 1;
                continue;
              }
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
                };

                container_content.push(wrap_node_with_paragraph(
                  &element.properties.clone(),
                  &[hast::Node::Text(hast::Text {
                    value: line.into(),
                    position: None,
                  })],
                ));
              }
            }
          }
        }
        // e.g. > [!tip]
        if element.tag_name == "blockquote" {
          // why use element.children.get(1)?
          // in crates/mdx_rs/mdast_util_to_hast.rs, method `transform_block_quote`
          // always insert Text { value: "\n".into(),  position: None } in blockquote's children
          if let Some(hast::Node::Element(first_element)) = element.children.get(1) {
            if first_element.tag_name == "p" {
              if let Some(hast::Node::Text(text)) = first_element.children.first() {
                if text.value.trim().starts_with("[!") {
                  // split data if previous step parse in one line
                  // e.g <p>[!TIP] this is a tip</p>
                  let (self_container_type, remaining_data) = parse_github_alerts_container_meta(&text.value);
                  if !is_valid_container_type(&self_container_type) {
                    index += 1;
                    continue;
                  }
                  // in this case, container_type as container_title
                  container_type = self_container_type.clone();
                  container_title = self_container_type.clone();
                  container_title.make_ascii_uppercase();

                  container_content_start = true;
                  container_content_start_index = index;

                  // reform paragraph tag
                  let mut paragraph_children = first_element.children.clone();
                  if remaining_data.len() > 0 {
                    paragraph_children[0] = hast::Node::Text(hast::Text {
                      value: remaining_data.into(),
                      position: None,
                    })
                  } else {
                    paragraph_children.remove(0);
                  }
                  // reform blockquote tag
                  let mut children = element.children.clone();

                  if paragraph_children.is_empty() {
                    children.remove(1);
                  } else {
                    children[1] =
                      wrap_node_with_paragraph(&element.properties.clone(), &paragraph_children)
                  }

                  container_content = children;

                  container_content_end = true;
                  container_content_end_index = index;
                }
              }
            }
          }
        }
      }

      // Collect the container content in current p tag
      if container_content_start && !container_content_end && !element.children.is_empty() {
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
          if !fragments.is_empty() {
            if index == container_content_start_index && !container_content.is_empty() {
              let first_node = container_content.first_mut().unwrap();
              let mut children = first_node.children().unwrap().to_vec();
              children.extend(fragments);
              *first_node.children_mut().unwrap() = children;
            } else {
              container_content.push(wrap_node_with_paragraph(
                &element.properties.clone(),
                &fragments,
              ));
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
      if let hast::Node::MdxExpression(_) = child {
        continue;
      }
      container_content.push(child.clone());
    }
  }
}

pub fn mdx_plugin_container(root: &mut hast::Node) {
  // 1. Traverse children, get all p tags, check if they start with :::
  // If it is, it is regarded as container syntax, and the content from the beginning of ::: to the end of a certain ::: is regarded as a container
  // 2. Traverse children, get all blockquote tags, check if they next child's first element is p tags and if start with [! and end of ]
  // If it is, it is regarded as container syntax, and the content from the beginning of blockquote to the end of a certain blockquote is regarded as a container
  // The element of this container is a div element, className is "rspress-directive"
  // for example:
  // :::tip
  // this is a tip
  // :::
  // or
  // > [!tip]
  // > this is a tip
  // Will be transformed to:
  // <div class="rspress-directive">
  //   <div class="rspress-directive-title">tip</div>
  //   <div class="rspress-directive-content">
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
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
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
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
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
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
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
 fn test_parse_github_alerts_container_meta() {
  assert_eq!(parse_github_alerts_container_meta("[!TIP]"), ("TIP".into(), "".into()));
  assert_eq!(parse_github_alerts_container_meta("[!TIP this is tip block"), ("".into(), "".into()));
  assert_eq!(parse_github_alerts_container_meta("[!TIP] this is tip block"), ("TIP".into(), " this is tip block".into()));
 }

  #[test]
  fn test_container_plugin_with_mdx_flow_in_content() {
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
        hast::Node::MdxJsxElement(hast::MdxJsxElement {
          name: Some("Rspack".into()),
          attributes: vec![],
          children: vec![],
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
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "tip".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
              )],
              children: vec![hast::Node::MdxJsxElement(hast::MdxJsxElement {
                name: Some("Rspack".into()),
                attributes: vec![],
                children: vec![],
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
  fn test_container_plugin_width_details_title() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![
        hast::Node::Element(hast::Element {
          tag_name: "p".into(),
          properties: vec![],
          children: vec![hast::Node::Text(hast::Text {
            value: ":::details Note".into(),
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
          tag_name: "details".into(),
          properties: vec![(
            "className".into(),
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "details".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "summary".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
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
  fn test_container_plugin_with_github_alerts_title() {
    let mut root = hast::Node::Root(hast::Root {
      children: vec![hast::Node::Element(hast::Element {
        tag_name: "blockquote".into(),
        properties: vec![],
        children: vec![
          hast::Node::Text(hast::Text {
            value: "\n".into(),
            position: None,
          }),
          hast::Node::Element(hast::Element {
            tag_name: "p".into(),
            properties: vec![],
            children: vec![hast::Node::Text(hast::Text {
              value: "[!TIP]".into(),
              position: None,
            })],
            position: None,
          }),
        ],
        position: None,
      })],
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
            hast::PropertyValue::SpaceSeparated(vec!["rspress-directive".into(), "TIP".into()])
          ),],
          children: vec![
            hast::Node::Element(hast::Element {
              tag_name: "div".into(),
              properties: vec![(
                "className".into(),
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-title".into()])
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
                hast::PropertyValue::SpaceSeparated(vec!["rspress-directive-content".into()])
              )],
              children: vec![
                hast::Node::Text(hast::Text {
                  value: "\n".into(),
                  position: None,
                }),
              ],
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
