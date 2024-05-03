//! Author: shulaoda
//!
//! This plugin is used to collect code lang in mdx.

use markdown::mdast::Node;
use std::collections::HashSet;

pub fn mdx_plugin_highlighter(node: &Node) -> Vec<String> {
  let mut languages: HashSet<String> = HashSet::new();

  if let Node::Root(root) = node {
    for child in &root.children {
      if let Node::Code(code) = child {
        if let Some(lang) = &code.lang {
          languages.insert(lang.clone());
        }
      }
    }
  }

  languages.into_iter().collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use markdown::mdast;

  #[test]
  fn test_mdx_plugin_highlighter() {
    let code = mdast::Node::Root(mdast::Root {
      children: vec![mdast::Node::Code(mdast::Code {
        lang: Some("markdown".into()),
        meta: None,
        value: "".into(),
        position: None,
      })],
      position: None,
    });

    assert_eq!(
      mdx_plugin_highlighter(&code),
      vec!["markdown".to_string()]
    );
  }
}
