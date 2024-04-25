//! Author: sanyuan0704
//!
//! This plugin is used to generate toc in mdx. Features:
//! 1. Collect h2 ~ h4 as toc
//! 2. Collect h1 as title
//! 3. Support custom id, example: `# hello world {#custom-id}`

use markdown::mdast::{self, Heading};
use slugger::Slugger;
use std::{fmt::format, vec};
use utils::extract_title_and_id;

#[derive(Debug, Clone)]
pub struct TocItem {
  pub text: String,
  pub depth: u8,
  pub id: String,
}

pub struct TocResult {
  pub title: String,
  pub toc: Vec<TocItem>,
}

pub enum NodeType {
  Link,
  Strong,
  Emphasis
}

pub fn extract_text_from_node(node: &mdast::Node, node_type: NodeType) -> String {
  match node_type {
    NodeType::Link => {
      if let mdast::Node::Link(link) = node {
        if let mdast::Node::Text(text) = &link.children[0] {
          return text.value.clone();
        }
      }
    }
    NodeType::Strong => {
      if let mdast::Node::Strong(strong) = node {
        if let mdast::Node::Text(text) = &strong.children[0] {
          return text.value.clone();
        }
      }
    }
    NodeType::Emphasis => {
      if let mdast::Node::Emphasis(emphasis) = node {
        // Emphasis&Strong: ***hello***
        if let mdast::Node::Strong(_) = &emphasis.children[0] {
          return format!("**{}**", extract_text_from_node(&emphasis.children[0], NodeType::Strong));
        }
        if let mdast::Node::Text(text) = &emphasis.children[0] {
          return text.value.clone();
        }
      }
    }
  }
  String::new()
}
pub fn collect_title_in_mdast(heading: &mut Heading) -> (String, String) {
  let mut title = String::new();
  let mut custom_id = String::new();
  for child in &mut heading.children {
    match child {
      mdast::Node::Text(text) => {
        // example: hello world {#custom-id}
        // Then we extract the `hello world` as title and `custom-id` as id
        let (title_part, id_part) = extract_title_and_id(&text.value);
        title.push_str(&title_part);
        custom_id = id_part;
      }
      mdast::Node::Strong(_) => {
        title.push_str(format!("**{}**", extract_text_from_node(child, NodeType::Strong)).as_str())
      }
      mdast::Node::Emphasis(_) => {
        title.push_str(format!("*{}*", extract_text_from_node(child, NodeType::Emphasis)).as_str())
      }
      mdast::Node::InlineCode(code) => title.push_str(format!("`{}`", code.value).as_str()),
      mdast::Node::Link(_) => {
        title.push_str(extract_text_from_node(child, NodeType::Link).as_str())
      }
      _ => continue, // Continue if node is not Text or Code
    }
  }
  (title, custom_id)
}

pub fn mdx_plugin_toc(node: &mut mdast::Node) -> TocResult {
  let mut toc: Vec<TocItem> = vec![];
  let mut title = String::new();
  let mut slugger = Slugger::new();
  if let mdast::Node::Root(root) = node {
    for child in &mut root.children {
      if let mdast::Node::Heading(heading) = child {
        let mut id;
        let toc_title;
        (toc_title, id) = collect_title_in_mdast(heading);

        if heading.depth == 1 {
          title = toc_title.clone();
        }
        if id.is_empty() {
          id = slugger.slug(&toc_title, false);
        }
        // Collect h2 ~ h4
        if heading.depth < 2 || heading.depth > 4 {
          continue;
        }
        toc.push(TocItem {
          text: toc_title,
          depth: heading.depth,
          id,
        });
      }
    }
  }

  TocResult { title, toc }
}

#[cfg(test)]
mod tests {
  use super::*;
  use markdown::mdast;

  #[test]
  fn test_collect_title_in_mdast() {
    let mut heading = mdast::Heading {
      depth: 1,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let mut heading2 = mdast::Heading {
      depth: 1,
      children: vec![
        mdast::Node::Strong(mdast::Strong {
          children: vec![mdast::Node::Text(mdast::Text {
            value: "Hello".to_string(),
            position: None,
          })],
          position: None,
        }),
        mdast::Node::Emphasis(mdast::Emphasis {
          children: vec![mdast::Node::Text(mdast::Text {
            value: "World".to_string(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    };
    let mut heading3 = mdast::Heading {
      depth: 1,
      children: vec![mdast::Node::Emphasis(mdast::Emphasis {
        children: vec![mdast::Node::Strong(mdast::Strong {
          children: vec![mdast::Node::Text(mdast::Text {
            value: "Hello World".to_string(),
            position: None,
          })],
          position: None,
        })],
        position: None,
      })],
      position: None,
    };

    assert_eq!(
      collect_title_in_mdast(&mut heading),
      ("Hello`World`".to_string(), "".to_string())
    );
    assert_eq!(
      collect_title_in_mdast(&mut heading2),
      ("**Hello***World*".to_string(), "".to_string())
    );
    assert_eq!(
      collect_title_in_mdast(&mut heading3),
      ("***Hello World***".to_string(), "".to_string())
    );
  }

  #[test]
  fn test_collect_title_in_mdast_with_custom_id() {
    let mut heading = mdast::Heading {
      depth: 1,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
        mdast::Node::Text(mdast::Text {
          value: " 123 {#custom-id}".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    assert_eq!(
      collect_title_in_mdast(&mut heading),
      ("Hello`World` 123".to_string(), "custom-id".to_string())
    );

    assert_eq!(
      heading.children[2],
      mdast::Node::Text(mdast::Text {
        value: " 123 {#custom-id}".to_string(),
        position: None,
      })
    );
  }

  #[test]
  fn test_mdx_plugin_toc() {
    let heading = mdast::Heading {
      depth: 1,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let heading2 = mdast::Heading {
      depth: 2,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let heading3 = mdast::Heading {
      depth: 3,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
        mdast::Node::Link(mdast::Link {
          url: "https://github.com".to_string(),
          title: None,
          children: vec![mdast::Node::Text(mdast::Text {
            value: "Github".to_string(),
            position: None,
          })],
          position: None,
        }),
      ],
      position: None,
    };
    let heading4 = mdast::Heading {
      depth: 4,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let heading5 = mdast::Heading {
      depth: 5,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let heading6 = mdast::Heading {
      depth: 6,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let heading7: Heading = mdast::Heading {
      depth: 4,
      children: vec![
        mdast::Node::Text(mdast::Text {
          value: "Hello".to_string(),
          position: None,
        }),
        mdast::Node::Strong(mdast::Strong {
          children: vec![mdast::Node::Text(mdast::Text {
            value: "abc".to_string(),
            position: None,
          })],
          position: None,
        }),
        mdast::Node::InlineCode(mdast::InlineCode {
          value: "World".to_string(),
          position: None,
        }),
      ],
      position: None,
    };
    let mut root = mdast::Node::Root(mdast::Root {
      children: vec![
        mdast::Node::Heading(heading),
        mdast::Node::Heading(heading2),
        mdast::Node::Heading(heading3),
        mdast::Node::Heading(heading4),
        mdast::Node::Heading(heading5),
        mdast::Node::Heading(heading6),
        mdast::Node::Heading(heading7),
      ],
      position: None,
    });

    let result = mdx_plugin_toc(&mut root);

    assert_eq!(result.title, "Hello`World`");
    assert_eq!(result.toc.len(), 4);
    assert_eq!(result.toc[1].text, "Hello`World`Github");
    assert_eq!(result.toc[3].text, "Hello**abc**`World`");
  }
}
