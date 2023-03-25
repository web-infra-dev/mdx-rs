# @modern-js/mdx-binding

This is a Node.js binding for MDX compliation of [Modern.js Doc](https://modernjs.dev/doc-tools) which is a modern documentation tool based on [Rspack](https://www.rspack.org/).

## Credits

Thanks to [mdxjs-rs](https://github.com/wooorm/mdxjs-rs), the awesome Rust library authored by [wooorm](https://github.com/wooorm).We forked this library and customize it for Modern.js Doc, including the following features:

- 🔥 Add Node.js binding so that we can use it in Node.js.
- 📝 [mdx_plugin_container]('./crates/plugin_container'): implement container grammer like `:::tip` in [@modern-js/remark-container](https://github.com/web-infra-dev/modern.js/tree/main/packages/toolkit/remark-container).
- 🔖 [mdx_plugin_code_block](./crates/plugin_code_block): custom code block DOM structure for Modern.js Doc.
- 📚 [mdx_plugin_toc](./crates/plugin_toc.rs): generate table of contents.
- 🎨 [mdx_plugin_frontmatter](./crates/plugin_frontmatter.rs): parse frontmatter and export it in the esm module.
- ✨[mdx_plugin_external_link](./crates/plugin_external_link.rs): add `target="_blank"` and `rel="noopener noreferrer"` to external link.
- 🍎 [mdx_plugin_header_anchor](./crates/plugin_header_anchor.rs): add anchor for every header.
- 🔨 [slugger](./crates/slugger): generate slug for header, port from [github-slugger](https://github.com/Flet/github-slugger).

Also, thanks to [napi-rs](https://github.com/napi-rs/napi-rs), authored by [Brooooooklyn](https://github.com/Brooooooklyn), which is a great solution to help us build Node.js binding for Rust.
