# @modern-js/mdx-rs-binding

This is a Node.js binding for MDX compliation of [Modern.js Doc](https://modernjs.dev/doc-tools) which is a modern documentation tool based on [Rspack](https://www.rspack.org/).

It can be 5~10x faster than compiler in pure JavaScript version.The [benchmark](./benches/index.mjs) result of `@modern-js/mdx-rs-binding` vs `@mdx-js/mdx` is as follows:

| Tool | Time |
| --- | ---- |
| @modern-js/mdx-rs-binding    | 537 ms |
| @mdx-js/mdx  | 3268 ms |

We forked [mdxjs-rs](https://github.com/wooorm/mdxjs-rs), the Rust version of mdx compiler and customize it for Modern.js Doc, adding the following features:

| Crate                                                       | Description                                                                                                                                                         |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [mdx_rs_binding](./crates/binding)                          | 🔥 Add Node.js binding so that we can use it in Node.js.                                                                                                            |
| [mdx_plugin_container](./crates/plugin_container)           | Implement container grammar like `:::tip` in [@modern-js/remark-container](https://github.com/web-infra-dev/modern.js/tree/main/packages/toolkit/remark-container). |
| [mdx_plugin_code_block](./crates/plugin_code_block)         | Custom code block DOM structure for Modern.js Doc.                                                                                                                  |
| [mdx_plugin_toc](./crates/plugin_toc)                       | Generate table of contents.                                                                                                                                         |
| [mdx_plugin_frontmatter](./crates/plugin_frontmatter)       | Parse frontmatter and export it in the esm module.                                                                                                                  |
| [mdx_plugin_external_link](./crates/plugin_external_link)   | Add `target="_blank"` and `rel="noopener noreferrer"` to external link.                                                                                             |
| [mdx_plugin_header_anchor](./crates/plugin_header_anchor)   | Add anchor for every header.                                                                                                                                        |
| [mdx_plugin_normalize_link](./crates/plugin_normalize_link) | Normalize link to complete url base on current filepath.                                                                                                            |
| [mdx_plugin_html](./crates/plugin_html)                     | Serialize hast to html string                                                                                                                                       |
| [slugger](./crates/slugger)                                 | Generate slug for header, port from [github-slugger](https://github.com/Flet/github-slugger).                                                                       |

## Credits

Thanks to [mdxjs-rs](https://github.com/wooorm/mdxjs-rs), the awesome Rust library authored by [wooorm](https://github.com/wooorm).

Also, thanks to [napi-rs](https://github.com/napi-rs/napi-rs), authored by [Brooooooklyn](https://github.com/Brooooooklyn), which is a great solution to help us build Node.js binding for Rust.
