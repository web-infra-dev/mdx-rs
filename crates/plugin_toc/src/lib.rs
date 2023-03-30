//! Author: sanyuan0704
//!
//! This plugin is used to generate toc in mdx.

use markdown::mdast;
use slugger::Slugger;

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

pub fn collect_title_in_mdast(node: &mdast::Node) -> String {
  let mut title = String::new();
  if let mdast::Node::Heading(heading) = node {
    for child in &heading.children {
      match child {
        mdast::Node::Text(text) => title.push_str(&text.value),
        mdast::Node::InlineCode(code) => title.push_str(&code.value),
        _ => continue, // Continue if node is not Text or Code
      }
    }
  }

  title.replace("\"", "\\\"").replace("'", "\\\'")
}

pub fn mdx_plugin_toc(node: &mut mdast::Node) -> TocResult {
  let mut toc: Vec<TocItem> = vec![];
  let mut title = String::new();
  let mut slugger = Slugger::new();
  if let mdast::Node::Root(root) = node {
    for child in &root.children {
      if let mdast::Node::Heading(heading) = child {
        if heading.depth == 1 {
          title = collect_title_in_mdast(child);
        }
        let toc_title = collect_title_in_mdast(child);
        let id = slugger.slug(&toc_title, false);
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
    // add toc exports in the module
    // such as `export const toc = [{ title: 'title', level: 1 }]`
    let title_exports = mdast::Node::MdxjsEsm(mdast::MdxjsEsm {
      value: format!("export const title = '{}';", title),
      position: None,
      stops: vec![],
    });
    let toc_code = toc
      .iter()
      .map(|item| {
        format!(
          "{{ text: \"{}\", depth: {}, id: \"{}\" }}",
          item.text, item.depth, item.id
        )
      })
      .collect::<Vec<String>>()
      .join(",");
    let toc_exports = mdast::Node::MdxjsEsm(mdast::MdxjsEsm {
      value: format!("export const toc = [{}];", toc_code),
      position: None,
      stops: vec![],
    });
    root.children.extend(vec![title_exports, toc_exports]);
  }

  TocResult { title, toc }
}

#[cfg(test)]
mod tests {
  use super::*;
  use markdown::mdast;

  #[test]
  fn test_collect_title_in_mdast() {
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
    assert_eq!(
      collect_title_in_mdast(&mdast::Node::Heading(heading)),
      "HelloWorld"
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
    let mut root = mdast::Node::Root(mdast::Root {
      children: vec![
        mdast::Node::Heading(heading),
        mdast::Node::Heading(heading2),
        mdast::Node::Heading(heading3),
        mdast::Node::Heading(heading4),
        mdast::Node::Heading(heading5),
        mdast::Node::Heading(heading6),
      ],
      position: None,
    });

    mdx_plugin_toc(&mut root);

    if let mdast::Node::Root(root) = root {
      assert_eq!(root.children.len(), 8);
      assert_eq!(
        root.children[6],
        mdast::Node::MdxjsEsm(mdast::MdxjsEsm {
          value: "export const title = 'HelloWorld';".to_string(),
          position: None,
          stops: vec![],
        })
      );
      assert_eq!(
        root.children[7],
        mdast::Node::MdxjsEsm(mdast::MdxjsEsm {
          value: "export const toc = [{ text: \"HelloWorld\", depth: 2, id: \"helloworld\" },{ text: \"HelloWorld\", depth: 3, id: \"helloworld-1\" },{ text: \"HelloWorld\", depth: 4, id: \"helloworld-2\" }];"
            .to_string(),
          position: None,
          stops: vec![],
        })
      );
    }
  }
}
