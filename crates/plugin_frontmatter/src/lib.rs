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

pub fn mdx_plugin_frontmatter(node: &mut mdast::Node) -> String {
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
    return yaml_to_json(&front_matter);
  }
  "{}".into()
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
}
