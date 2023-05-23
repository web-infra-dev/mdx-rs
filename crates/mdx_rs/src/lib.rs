//! Public API of `mdxjs-rs`.
//!
//! This module exposes primarily [`compile()`][].
//!
//! *   [`compile()`][]
//!     — turn MDX into JavaScript
#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

extern crate markdown;
mod configuration;
mod hast_util_to_swc;
mod mdast_util_to_hast;
mod mdx_plugin_recma_document;
mod mdx_plugin_recma_jsx_rewrite;
mod swc;
mod swc_util_build_jsx;
mod swc_utils;

use crate::{
  hast_util_to_swc::hast_util_to_swc,
  mdast_util_to_hast::mdast_util_to_hast,
  mdx_plugin_recma_document::{mdx_plugin_recma_document, Options as DocumentOptions},
  mdx_plugin_recma_jsx_rewrite::{mdx_plugin_recma_jsx_rewrite, Options as RewriteOptions},
  swc::{parse_esm, serialize},
  swc_util_build_jsx::{swc_util_build_jsx, Options as BuildOptions},
};
use hast;
use markdown::{to_mdast, Constructs, Location, ParseOptions};
use mdx_plugin_code_block::mdx_plugin_code_block;
use mdx_plugin_container::mdx_plugin_container;
use mdx_plugin_external_link::mdx_plugin_external_link;
use mdx_plugin_frontmatter::mdx_plugin_frontmatter;
use mdx_plugin_header_anchor::mdx_plugin_header_anchor;
use mdx_plugin_html::mdx_plugin_html;
use mdx_plugin_normalize_link::mdx_plugin_normalize_link;
use mdx_plugin_toc::{mdx_plugin_toc, TocItem};

pub use crate::configuration::{MdxConstructs, MdxParseOptions, Options};
pub use crate::mdx_plugin_recma_document::JsxRuntime;

pub struct CompileResult {
  pub code: String,
  pub links: Vec<String>,
  pub html: String,
  pub title: String,
  pub toc: Vec<TocItem>,
}

pub fn compile(
  value: &String,
  filepath: &String,
  development: bool,
  root: &String,
) -> CompileResult {
  let is_mdx = filepath.ends_with(".mdx");
  let parse_options = ParseOptions {
    constructs: Constructs {
      frontmatter: true,
      // Enable GFM Grammer
      gfm_autolink_literal: true,
      gfm_label_start_footnote: true,
      gfm_footnote_definition: true,
      gfm_strikethrough: true,
      gfm_table: true,
      gfm_task_list_item: true,
      // If is_mdx is true, use mdx constructs, or use markdown constructs
      ..if is_mdx {
        Constructs::mdx()
      } else {
        Constructs::default()
      }
    },
    gfm_strikethrough_single_tilde: false,
    math_text_single_dollar: false,
    mdx_esm_parse: Some(Box::new(parse_esm)),
    mdx_expression_parse: None,
  };
  let document_options = DocumentOptions {
    pragma: Some("React.createElement".to_string()),
    pragma_frag: Some("React.Fragment".to_string()),
    pragma_import_source: Some("react".to_string()),
    jsx_import_source: Some("react".to_string()),
    jsx_runtime: Some(JsxRuntime::Automatic),
  };
  let rewrite_options = RewriteOptions {
    development,
    provider_import_source: Some("@mdx-js/react".to_string()),
  };
  let build_options = BuildOptions { development };

  let location = Location::new(value.as_bytes());
  let mut mdast =
    to_mdast(value.as_str(), &parse_options).expect(format!("value: {}", value).as_str());
  let toc_result = mdx_plugin_toc(&mut mdast);
  mdx_plugin_frontmatter(&mut mdast);
  let mut hast = mdast_util_to_hast(&mdast);
  mdx_plugin_header_anchor(&mut hast);
  mdx_plugin_container(&mut hast);
  mdx_plugin_external_link(&mut hast);
  let links = mdx_plugin_normalize_link(&mut hast, root, filepath);
  let html = mdx_plugin_html(&hast);
  let mut program = hast_util_to_swc(&hast, Some(filepath.to_string()), Some(&location))
    .expect(format!("file: {}", filepath).as_str());
  mdx_plugin_recma_document(&mut program, &document_options, Some(&location))
    .expect(format!("file: {}", filepath,).as_str());
  mdx_plugin_recma_jsx_rewrite(&mut program, &rewrite_options, Some(&location));

  swc_util_build_jsx(&mut program, &build_options, Some(&location)).unwrap();
  let code = serialize(&mut program.module, Some(&program.comments));
  CompileResult {
    code,
    links,
    html,
    title: toc_result.title,
    toc: toc_result.toc,
  }
}
