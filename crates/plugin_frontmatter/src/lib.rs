//! Author: sanyuan0704
//!
//! This plugin is used to parse the front matter in markdown and export it in mdx file.
use markdown::mdast;
use serde_yaml::Value;

use serde_json;

fn yaml_to_json(yaml_str: &str) -> String {
  if yaml_str.is_empty() {
    return "{}".into();
  }
  let parsed_value: Value =
    serde_yaml::from_str(yaml_str).expect(format!("Failed to parse yaml: {}。", yaml_str).as_str());
  serde_json::to_string(&parsed_value).unwrap()
}

pub fn mdx_plugin_frontmatter(node: &mut mdast::Node) {
  if let mdast::Node::Root(root) = node {
    let mut front_matter = String::new();
    let mut front_matter_node_index = None;
    for (i, child) in root.children.iter().enumerate() {
      if let mdast::Node::Yaml(yaml) = child {
        front_matter_node_index = Some(i);
        front_matter = yaml.value.clone();
      }
    }
    if let Some(i) = front_matter_node_index {
      root.children.remove(i);
    }
    let front_matter_exports = mdast::Node::MdxjsEsm(mdast::MdxjsEsm {
      value: format!("export const frontmatter = {}", yaml_to_json(&front_matter)),
      position: None,
      stops: vec![],
    });
    root.children.insert(0, front_matter_exports);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_yaml_to_json_empty() {
    let yaml = "";
    let expected = "{}".to_string();
    assert_eq!(yaml_to_json(yaml), expected);
  }

  #[test]
  fn test_yaml_to_json_simple() {
    let yaml = "
name: John Doe
age: 30
";
    let expected = r#"{"name":"John Doe","age":30}"#.to_string();
    assert_eq!(yaml_to_json(yaml), expected);
  }

  #[test]
  fn test_yaml_to_json_nested() {
    let yaml = "
person:
  name: John Doe
  age: 30
";
    let expected = r#"{"person":{"name":"John Doe","age":30}}"#.to_string();
    assert_eq!(yaml_to_json(yaml), expected);
  }

  #[test]
  fn test_mdx_plugin_front_matter() {
    use markdown::mdast::{MdxjsEsm, Node, Root, Yaml};

    // Input AST
    let mut ast = Node::Root(Root {
      children: vec![Node::Yaml(Yaml {
        value: "title: My Blog\nauthor: John Doe\n".to_string(),
        position: None,
      })],
      position: None,
    });

    // Expected output AST
    let expected_ast = Node::Root(Root {
      children: vec![Node::MdxjsEsm(MdxjsEsm {
        value: "export const frontmatter = {\"title\":\"My Blog\",\"author\":\"John Doe\"}"
          .to_string(),
        position: None,
        stops: vec![],
      })],
      position: None,
    });

    // Apply function and check output
    mdx_plugin_frontmatter(&mut ast);
    assert_eq!(ast, expected_ast);
  }
}
