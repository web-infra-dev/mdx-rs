use std::path::Path;

use hast;

const PROTOCOLS: &[&str] = &["http://", "https://", "mailto:", "tel:", "javascript:", "#"];

fn normalize_link(url: &String, root: &String, filepath: &String) -> String {
  // If url includes following case, return directly
  // http/https、mailto、tel、javascript、#
  if PROTOCOLS.iter().any(|protocol| url.starts_with(protocol)) {
    return url.to_owned();
  }

  // parse extname and remove it
  let mut url = url.to_string();
  let root_path = Path::new(root);
  let file_path = Path::new(filepath);

  if let Ok(relative_path) = file_path.strip_prefix(root_path) {
    if url.starts_with(".") {
      let mut base_dir = relative_path.parent().unwrap();

      if url.starts_with("./") {
        url = url.replace("./", "");
      }

      while url.starts_with("../") {
        url = url.replace("../", "");
        base_dir = base_dir.parent().unwrap();
      }

      url = base_dir.join(Path::new(&url)).to_str().unwrap().to_string();
    }

    // remove the starting slash
    if url.starts_with('/') {
      url = url[1..].to_string();
    }

    // ensure the url starts with /
    if !url.starts_with('/') {
      url = format!("/{}", url);
    }

    let extname = match url.rfind('.') {
      Some(index) => url[index..].to_string(),
      None => "".to_string(),
    };

    // remove extname
    if !extname.is_empty() {
      url = url.replace(&extname, "");
    }
  }

  url
}

fn mdx_plugin_normalize_link_impl(
  node: &mut hast::Node,
  root: &String,
  filepath: &String,
) -> Vec<String> {
  let mut links = vec![];
  match node {
    hast::Node::Root(root_node) => {
      for child in root_node.children.iter_mut() {
        links.append(&mut mdx_plugin_normalize_link_impl(child, root, filepath));
      }
    }
    hast::Node::Element(element) => {
      if element.tag_name == "a" {
        // Get the href property
        let href = element.properties.iter().find(|(key, _)| key == "href");
        if let Some(href) = href {
          if let (_, hast::PropertyValue::String(href)) = href {
            let normalized_link = normalize_link(href, root, filepath);
            links.push(normalized_link.clone());
            // replace the href property
            element
              .properties
              .iter_mut()
              .find(|(key, _)| key == "href")
              .unwrap()
              .1 = hast::PropertyValue::String(normalized_link);
          }
        }
      }
      for child in element.children.iter_mut() {
        links.append(&mut mdx_plugin_normalize_link_impl(child, root, filepath));
      }
    }
    _ => {}
  }
  links
}

pub fn mdx_plugin_normalize_link(
  node: &mut hast::Node,
  root: &String,
  filepath: &String,
) -> Vec<String> {
  mdx_plugin_normalize_link_impl(node, root, filepath)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_external_link() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    assert_eq!(
      normalize_link(&"http://example.com".to_string(), &root, &filepath),
      "http://example.com".to_string()
    );
    assert_eq!(
      normalize_link(&"https://example.com".to_string(), &root, &filepath),
      "https://example.com".to_string()
    );
    assert_eq!(
      normalize_link(&"mailto:xxx.com".to_string(), &root, &filepath),
      "mailto:xxx.com".to_string()
    );
    assert_eq!(
      normalize_link(&"tel:xxx.com".to_string(), &root, &filepath),
      "tel:xxx.com".to_string()
    );
    assert_eq!(
      normalize_link(&"javascript:void(0)".to_string(), &root, &filepath,),
      "javascript:void(0)".to_string()
    );
    assert_eq!(
      normalize_link(&"#aaa".to_string(), &root, &filepath),
      "#aaa".to_string()
    );
  }
}
