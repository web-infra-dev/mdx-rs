use std::path::Path;

use hast;

fn normalize_link(url: &String, root: &String, filepath: &String, default_lang: &String) -> String {
  // If url includes following case, return directly
  // http/https、mailto、tel、javascript、#
  if url.starts_with("http://")
    || url.starts_with("https://")
    || url.starts_with("mailto:")
    || url.starts_with("tel:")
    || url.starts_with("javascript:")
    || url.starts_with("#")
  {
    return url.to_string();
  }
  // parse extname and remove it
  let mut url = url.to_string();
  let root_path = Path::new(root);
  let file_path = Path::new(filepath);
  let relative_path = file_path.strip_prefix(root_path).unwrap();
  let relative_path_str = relative_path.to_str().unwrap();
  // use path separator to parse the first part of the relative filepath, regart it as the lang
  let lang = match relative_path_str.find(std::path::MAIN_SEPARATOR) {
    Some(index) => relative_path_str[..index].to_string(),
    None => "".to_string(),
  };

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

  if &lang == default_lang {
    url = url.replace(&format!("{}/", lang), "");
  } else {
    // if the url doesn't start with lang, add it
    if !url.starts_with(&format!("{}/", lang)) {
      url = format!("{}/{}", lang, url);
    }
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

  url
}

fn mdx_plugin_normalize_link_impl(
  node: &mut hast::Node,
  root: &String,
  filepath: &String,
  default_lang: &String,
) -> Vec<String> {
  let mut links = vec![];
  match node {
    hast::Node::Root(root_node) => {
      for child in root_node.children.iter_mut() {
        links.append(&mut mdx_plugin_normalize_link_impl(
          child,
          root,
          filepath,
          default_lang,
        ));
      }
    }
    hast::Node::Element(element) => {
      if element.tag_name == "a" {
        // Get the href property
        let href = element.properties.iter().find(|(key, _)| key == "href");
        if let Some(href) = href {
          if let (_, hast::PropertyValue::String(href)) = href {
            let normalized_link = normalize_link(href, root, filepath, default_lang);
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
        links.append(&mut mdx_plugin_normalize_link_impl(
          child,
          root,
          filepath,
          default_lang,
        ));
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
  default_lang: &String,
) -> Vec<String> {
  mdx_plugin_normalize_link_impl(node, root, filepath, default_lang)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_external_link() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    let default_lang = "en".to_string();

    assert_eq!(
      normalize_link(
        &"http://example.com".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "http://example.com".to_string()
    );
    assert_eq!(
      normalize_link(
        &"https://example.com".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "https://example.com".to_string()
    );
    assert_eq!(
      normalize_link(
        &"mailto:xxx.com".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "mailto:xxx.com".to_string()
    );
    assert_eq!(
      normalize_link(&"tel:xxx.com".to_string(), &root, &filepath, &default_lang),
      "tel:xxx.com".to_string()
    );
    assert_eq!(
      normalize_link(
        &"javascript:void(0)".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "javascript:void(0)".to_string()
    );
    assert_eq!(
      normalize_link(&"#aaa".to_string(), &root, &filepath, &default_lang),
      "#aaa".to_string()
    );
  }

  #[test]
  fn test_normalize_link() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    let mut default_lang = "zh".to_string();

    assert_eq!(
      normalize_link(
        &"zh/guide/config.md".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(&"zh/config".to_string(), &root, &filepath, &default_lang),
      "/config".to_string()
    );
    assert_eq!(
      normalize_link(&"en/config".to_string(), &root, &filepath, &default_lang),
      "/en/config".to_string()
    );

    assert_eq!(
      normalize_link(&"./model.md".to_string(), &root, &filepath, &default_lang),
      "/guide/model".to_string()
    );

    assert_eq!(
      normalize_link(
        &"../api/component.md".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "/api/component".to_string()
    );

    default_lang = "en".to_string();

    assert_eq!(
      normalize_link(
        &"../api/component.md".to_string(),
        &root,
        &filepath,
        &default_lang
      ),
      "/zh/api/component".to_string()
    );
  }

  #[test]
  fn test_extract_links() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    let default_lang = "zh".to_string();
    let mut node = hast::Node::Root(hast::Root {
      children: vec![hast::Node::Element(hast::Element {
        tag_name: "a".to_string(),
        properties: vec![(
          "href".to_string(),
          hast::PropertyValue::String("http://example.com".to_string()),
        )],
        children: vec![],
        position: None,
      })],
      position: None,
    });
    let links = mdx_plugin_normalize_link(&mut node, &root, &filepath, &default_lang);
    assert_eq!(links, vec!["http://example.com"]);
  }
}
