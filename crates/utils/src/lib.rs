//! Author: sanyuan0704

pub fn extract_title_and_id(text_value: &str) -> (String, String) {
  let mut title = String::new();
  let mut custom_id = String::new();
  if let Some(index) = text_value.find("{#") {
    let (mut title_part, id_part) = text_value.split_at(index);
    title_part = title_part.trim_end();
    title.push_str(title_part);
    let id_part = id_part[2..].to_string();
    if let Some(index) = id_part.find("}") {
      let (id_part, _) = id_part.split_at(index);
      custom_id.push_str(id_part);
    }
  } else {
    title.push_str(&text_value);
  }
  title = title.replace("\"", "\\\"").replace("'", "\\\'");
  (title, custom_id)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extract_title_and_id_with_custom_id() {
    let (title, custom_id) = extract_title_and_id("Hello World {#id123}");
    assert_eq!(title, "Hello World");
    assert_eq!(custom_id, "id123");
  }

  #[test]
  fn test_extract_title_and_id_without_custom_id() {
    let (title, custom_id) = extract_title_and_id("Hello World");
    assert_eq!(title, "Hello World");
    assert_eq!(custom_id, "");
  }

  #[test]
  fn test_extract_title_and_id_with_quotes() {
    let (title, custom_id) = extract_title_and_id("\"Hello' World\" {#id123}");
    assert_eq!(title, "\\\"Hello\\' World\\\"");
    assert_eq!(custom_id, "id123");
  }
}
