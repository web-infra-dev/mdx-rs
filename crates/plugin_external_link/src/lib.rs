//! Author: sanyuan0704
//!
//! This plugin is used to handle the external link in mdx.
//!
//! If the link is external, we will add the `target="_blank"` and `rel="noopener noreferrer"` attribute to the link element.

fn is_external_url(url: &str) -> bool {
  if url.starts_with("http://") || url.starts_with("https://") {
    return true;
  }
  false
}

fn transform_link_element(node: &mut hast::Node) {
  if let hast::Node::Element(node) = node {
    if node.tag_name == "a" {
      if let Some((_, hast::PropertyValue::String(url))) =
        node.properties.iter().find(|(key, _)| key == "href")
      {
        if is_external_url(url) {
          node.properties.push((
            "target".into(),
            hast::PropertyValue::String("_blank".into()),
          ));
          node.properties.push((
            "rel".into(),
            hast::PropertyValue::String("noopener noreferrer".into()),
          ));
        }
      }
    }
  }
}

fn mdx_plugin_external_link_impl(node: &mut hast::Node) {
  transform_link_element(node);

  if let Some(children) = node.children_mut() {
    for child in children {
      mdx_plugin_external_link_impl(child);
    }
  }
}

pub fn mdx_plugin_external_link(node: &mut hast::Node) {
  mdx_plugin_external_link_impl(node);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_external_url() {
    assert!(is_external_url("http://example.com"));
    assert!(is_external_url("https://example.com"));
    assert!(!is_external_url("doc/zh/config"));
    assert!(!is_external_url("http:/example.com"));
    assert!(!is_external_url("https:/example.com"));
  }

  #[test]
  fn test_transform_link_element() {
    let mut node = hast::Node::Element(hast::Element {
      tag_name: "a".into(),
      properties: vec![(
        "href".into(),
        hast::PropertyValue::String("http://example.com".into()),
      )],
      children: vec![],
      position: None,
    });
    transform_link_element(&mut node);
    assert_eq!(
      node,
      hast::Node::Element(hast::Element {
        tag_name: "a".into(),
        properties: vec![
          (
            "href".into(),
            hast::PropertyValue::String("http://example.com".into()),
          ),
          (
            "target".into(),
            hast::PropertyValue::String("_blank".into())
          ),
          (
            "rel".into(),
            hast::PropertyValue::String("noopener noreferrer".into()),
          ),
        ],
        children: vec![],
        position: None,
      })
    );
  }
}
