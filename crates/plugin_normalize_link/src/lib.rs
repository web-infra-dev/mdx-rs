use hast;
use std::path::Path;

const PROTOCOLS: &[&str] = &["http://", "https://", "mailto:", "tel:", "javascript:", "#"];
const TEMP_VARIABLE: &str = "image_";
const IMAGE_EXTNAMES: &[&str] = &[".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp"];
const MD_EXTNAMES: &[&str] = &[".md", ".mdx"];

fn generate_ast_import(
  index: usize,
  root: &String,
  src: &String,
  filepath: &String,
) -> hast::MdxjsEsm {
  let mut import_path = src.to_string();
  if import_path.starts_with(".") {
    import_path = normalize_link(src, root, &filepath);
  }
  hast::MdxjsEsm {
    value: format!(
      "import {} from \"{}\";",
      format!("{}{}", TEMP_VARIABLE, index),
      import_path
    ),
    position: None,
    stops: vec![],
  }
}

fn normalize_link(url: &String, root: &String, filepath: &String) -> String {
  // If url includes following case, return directly
  // http/https、mailto、tel、javascript、#
  if PROTOCOLS.iter().any(|protocol| url.starts_with(protocol)) {
    return url.to_owned();
  }
  let raw_url = url.to_string();
  // parse extname and remove it
  let mut url = url.to_string();
  let root_path = Path::new(root);
  let file_path = Path::new(filepath);
  // find the extname(before hash)
  // first, find the hash
  let hash_index = url.rfind('#').or_else(|| Some(url.len())).unwrap();
  // then, find the extname
  let extname = match url[..hash_index].rfind('.') {
    Some(index) => url[index..hash_index].to_string().to_lowercase(),
    None => "".to_string(),
  };

  let is_image = IMAGE_EXTNAMES.contains(&extname.as_str());
  let is_md = MD_EXTNAMES.contains(&extname.as_str());

  if let Ok(relative_path) = file_path.strip_prefix(root_path) {
    if url.starts_with(".") {
      // If the url is a image and relative path, return directly
      if is_image {
        return url;
      }
      let mut base_dir = relative_path.parent().unwrap();

      if url.starts_with("./") {
        url = url.replace("./", "");
      }

      while url.starts_with("../") {
        // only replace the first ../
        url.replace_range(0..3, "");
        match base_dir.parent() {
          Some(parent) => base_dir = parent,
          None => {
            println!(
              "Warning: The link is invalid: {} because the target path is out of the root dir: {}",
              raw_url, root
            );
            break;
          }
        }
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

    // remove md and mdx extname
    if !extname.is_empty() && is_md {
      url = url.replace(&extname, "");
    }
  }

  // Replace all the \\ to / in windows
  url.replace("\\", "/")
}

fn mdx_plugin_normalize_link_impl(
  node: &mut hast::Node,
  root: &String,
  filepath: &String,
  images: &mut Vec<hast::MdxjsEsm>,
) -> Vec<String> {
  let mut links = vec![];
  match node {
    hast::Node::Root(root_node) => {
      for child in root_node.children.iter_mut() {
        links.append(&mut mdx_plugin_normalize_link_impl(
          child, root, filepath, images,
        ));
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

      if element.tag_name == "img" {
        // Get the src and alt property
        let src = element.properties.iter().find(|(key, _)| key == "src");
        let alt = element.properties.iter().find(|(key, _)| key == "alt");
        // Then we will generate a mdxjsEsm node to import the image and push it into images
        if let Some(src) = src {
          if let (_, hast::PropertyValue::String(src)) = src {
            if PROTOCOLS.iter().any(|protocol| src.starts_with(protocol)) || src.starts_with('/') {
              return links;
            }
            let index = images.len();
            images.push(generate_ast_import(index, root, src, filepath));
            // Here we have to transform the element type to MdxJsxElement instead of replace src property
            // because the hast parser will parse the src property as hast::PropertyValue::String
            // and we can't get the original value
            let new_node = hast::Node::MdxJsxElement(hast::MdxJsxElement {
              name: Some("img".to_string()),
              attributes: vec![
                hast::AttributeContent::Property(hast::MdxJsxAttribute {
                  name: "src".to_string(),
                  value: Some(hast::AttributeValue::Expression(
                    markdown::mdast::AttributeValueExpression {
                      value: format!("{}{}", TEMP_VARIABLE, index),
                      stops: vec![],
                    },
                  )),
                }),
                hast::AttributeContent::Property(hast::MdxJsxAttribute {
                  name: "alt".to_string(),
                  value: alt.map(|(_, value)| match value {
                    hast::PropertyValue::String(v) => hast::AttributeValue::Literal(v.to_string()),
                    _ => hast::AttributeValue::Literal("".to_string()),
                  }),
                }),
              ],
              children: element.children.clone(),
              position: None,
            });

            *node = new_node;
          }
        }
      }

      if let Some(children) = node.children_mut() {
        for child in children {
          links.append(&mut mdx_plugin_normalize_link_impl(
            child, root, filepath, images,
          ));
        }
      }
    }
    hast::Node::MdxJsxElement(element) => {
      if let Some(name) = &element.name {
        if name != "img" {
          return links;
        }
        // Get the src property
        let src: Option<&mut hast::AttributeContent> =
          element.attributes.iter_mut().find(|attr| match attr {
            hast::AttributeContent::Property(property) => property.name == "src",
            _ => false,
          });
        // Add import statement and replace the src property
        if let Some(src) = src {
          if let hast::AttributeContent::Property(property) = src {
            if let Some(hast::AttributeValue::Literal(value)) = &mut property.value {
              if PROTOCOLS.iter().any(|protocol| value.starts_with(protocol))
                || value.starts_with('/')
              {
                return links;
              }
              let index = images.len();
              images.push(generate_ast_import(index, root, value, filepath));
              property.value = Some(hast::AttributeValue::Expression(
                markdown::mdast::AttributeValueExpression {
                  value: format!("{}{}", TEMP_VARIABLE, index),
                  stops: vec![],
                },
              ));
            }
          }
        }
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
  let mut images: Vec<hast::MdxjsEsm> = vec![];
  let links = mdx_plugin_normalize_link_impl(node, root, filepath, &mut images);
  if let hast::Node::Root(root) = node {
    // insert the images into the front of root node children
    for image in images {
      root.children.insert(0, hast::Node::MdxjsEsm(image));
    }
  }
  links
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

  #[test]
  fn test_relative_link() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    assert_eq!(
      normalize_link(&"./guide/config.md".to_string(), &root, &filepath),
      "/zh/guide/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(&"../guide/config.md".to_string(), &root, &filepath),
      "/zh/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(&"../../guide/config.md".to_string(), &root, &filepath),
      "/guide/config".to_string()
    );
  }

  #[test]
  fn test_link_with_hash() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    assert_eq!(
      normalize_link(&"./guide/config.html#aaa".to_string(), &root, &filepath),
      "/zh/guide/guide/config.html#aaa".to_string()
    );
    assert_eq!(
      normalize_link(
        &"./guide/config.html#tools.aaa".to_string(),
        &root,
        &filepath
      ),
      "/zh/guide/guide/config.html#tools.aaa".to_string()
    );
  }

  #[test]
  fn test_absolute_link() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    assert_eq!(
      normalize_link(&"/zh/guide/config.md".to_string(), &root, &filepath),
      "/zh/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(&"/zh/guide/config".to_string(), &root, &filepath),
      "/zh/guide/config".to_string()
    );
  }

  #[test]
  fn test_img_element() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    let mut node = hast::Node::Root(hast::Root {
      children: vec![hast::Node::MdxJsxElement(hast::MdxJsxElement {
        name: Some("img".to_string()),
        attributes: vec![hast::AttributeContent::Property(hast::MdxJsxAttribute {
          name: "src".to_string(),
          value: Some(hast::AttributeValue::Literal(
            "../../assets/a.png".to_string(),
          )),
        })],
        children: vec![],
        position: None,
      })],
      position: None,
    });
    let links = mdx_plugin_normalize_link(&mut node, &root, &filepath);
    assert_eq!(links.len(), 0);
    if let hast::Node::Root(root) = node {
      assert_eq!(root.children.len(), 2);
      if let hast::Node::MdxjsEsm(esm) = &root.children[0] {
        assert_eq!(
          esm.value,
          "import image_0 from \"../../assets/a.png\";".to_string()
        );
      }
      if let hast::Node::MdxJsxElement(element) = &root.children[1] {
        assert_eq!(element.name, Some("img".to_string()));
        if let hast::AttributeContent::Property(property) = &element.attributes[0] {
          assert_eq!(property.name, "src".to_string());
          if let Some(hast::AttributeValue::Expression(expression)) = &property.value {
            assert_eq!(expression.value, "image_0".to_string());
          }
        }
      }
    }
  }

  #[test]
  fn test_remove_extname() {
    let root = "/Users/xxx/xxx/xxx/docs".to_string();
    let filepath = "/Users/xxx/xxx/xxx/docs/zh/guide/config.md".to_string();
    assert_eq!(
      normalize_link(&"./guide/config.md".to_string(), &root, &filepath),
      "/zh/guide/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(&"./guide/config.mdx".to_string(), &root, &filepath),
      "/zh/guide/guide/config".to_string()
    );
    assert_eq!(
      normalize_link(
        &"./guide/config/webpack.resolve.alias".to_string(),
        &root,
        &filepath
      ),
      "/zh/guide/guide/config/webpack.resolve.alias".to_string()
    );
  }
}
