//! You can understand the module as `github-slugger-rs`
//!
//! Author: sanyuan0704
//!
//! Port of <https://github.com/Flet/github-slugger>
use regex::Regex;
use std::collections::HashMap;

pub struct Slugger {
  occurrences: HashMap<String, i32>,
}

fn normalize_slug(value: &String) -> String {
  let remove_re: Regex = Regex::new(r"[\p{Other_Number}\p{Close_Punctuation}\p{Final_Punctuation}\p{Initial_Punctuation}\p{Open_Punctuation}\p{Other_Punctuation}\p{Dash_Punctuation}\p{Symbol}\p{Control}\p{Private_Use}\p{Format}\p{Unassigned}\p{Separator}]").unwrap();
  let s = remove_re.replace_all(&value, |caps: &regex::Captures| {
    let c = caps.get(0).unwrap().as_str();
    if c == " " || c == "-" {
      "-".to_string()
    } else if c.chars().all(|a| a.is_alphabetic()) {
      c.to_string()
    } else {
      "".to_string()
    }
  });
  s.replace(|c: char| c.is_whitespace(), "-")
}

impl Slugger {
  /**
   * Create a new slug class.
   */
  pub fn new() -> Self {
    Slugger {
      occurrences: HashMap::new(),
    }
  }

  /**
   * Generate a unique slug.
   *
   * Tracks previously generated slugs: repeated calls with the same value
   * will result in different slugs.
   * Use the `slug` function to get same slugs.
   */
  pub fn slug(&mut self, value: &String, maintain_case: bool) -> String {
    let mut result = if maintain_case {
      value.clone()
    } else {
      value.to_lowercase()
    };
    let original_slug = result.clone();

    while self.occurrences.contains_key(&result) {
      let count = self.occurrences.get_mut(&original_slug).unwrap();
      *count += 1;
      result = format!("{}-{}", &original_slug, count);
    }

    self.occurrences.insert(result.clone(), 0);

    normalize_slug(&result)
  }

  /**
   * Reset - Forget all previous slugs
   *
   * @return ()
   */
  pub fn reset(&mut self) {
    self.occurrences.clear();
  }
}

/**
 * Generate a slug.
 *
 * Does not track previously generated slugs: repeated calls with the same value
 * will result in the exact same slug.
 * Use the `GithubSlugger` class to get unique slugs.
 *
 * @param  {String} value
 *   String of text to slugify
 * @param  {bool} [maintain_case=false]
 *   Keep the current case, otherwise make all lowercase
 * @return {String}
 *   A unique slug string
 */
pub fn slug(value: &String, maintain_case: bool) -> String {
  let result = if maintain_case {
    value.clone()
  } else {
    value.to_lowercase()
  };
  normalize_slug(&result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_slugger() {
    let mut slugger = Slugger::new();
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world"
    );
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world-1"
    );
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world-2"
    );
  }

  #[test]
  fn test_slugger_maintain_case() {
    let mut slugger = Slugger::new();
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), true),
      "Hello-World"
    );
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), true),
      "Hello-World-1"
    );
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), true),
      "Hello-World-2"
    );
  }

  #[test]
  fn test_slugger_reset() {
    let mut slugger = Slugger::new();
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world"
    );
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world-1"
    );
    slugger.reset();
    assert_eq!(
      slugger.slug(&"Hello World".to_string(), false),
      "hello-world"
    );
  }

  #[test]
  fn test_slug() {
    assert_eq!(slug(&"Hello World".to_string(), false), "hello-world");
    assert_eq!(slug(&"Hello World".to_string(), false), "hello-world");
    assert_eq!(
      slug(&"export 'function'".to_string(), false),
      "export-function"
    );
  }
}
