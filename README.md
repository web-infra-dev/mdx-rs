# @modern-js/mdx-rs-binding

This is a Node.js binding for MDX compliation of [Modern.js Doc](https://modernjs.dev/doc-tools) which is a modern documentation tool based on [Rspack](https://www.rspack.org/).

It can be 5~10x faster than compiler in pure JavaScript version.The [benchmark](./benches/index.mjs) result of `@modern-js/mdx-rs-binding` vs `@mdx-js/mdx` is as follows:

| Tool | Time Spend |
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


## Install

```bash
# npm
npm install @modern-js/mdx-rs-binding
# yarn
yarn add @modern-js/mdx-rs-binding
# pnpm
pnpm install @modern-js/mdx-rs-binding
```

## Usage

```js
import { compile } from '@modern-js/mdx-rs-binding';

async function main() {
  const value = `
  # Hello World

  This is a demo of @modern-js/mdx-rs-binding
  `;

  const result = await compile({
    // The mdx content
    value,
    // File path of the mdx file, the compiler will determine the different syntax(md/mdx) based on the file extension
    filepath: "xxx.mdx",
    // Whether to enable development mode, default is false
    development: true,
    // Current working directory, can be empty string
    root: "",
  });

  console.log(result);
}
```

Of course, you can also the `compileSync` function to compile mdx synchronously, which is not recommended because it will block the event loop and slow down the compile process.


```js
import { compileSync } from '@modern-js/mdx-rs-binding';

function main() {
  const value = `
  # Hello World

  This is a demo of @modern-js/mdx-rs-binding
  `;

  const result = compileSync({
    // The mdx content
    value,
    // File path of the mdx file, can be empty string
    filepath: "",
    // Whether to enable development mode, default is false
    development: true,
    // Current working directory, can be empty string
    root: "",
  });

  console.log(result);
}
```


## Credits

Thanks to [mdxjs-rs](https://github.com/wooorm/mdxjs-rs), the awesome Rust library authored by [wooorm](https://github.com/wooorm).

Also, thanks to [napi-rs](https://github.com/napi-rs/napi-rs), authored by [Brooooooklyn](https://github.com/Brooooooklyn), which is a great solution to help us build Node.js binding for Rust.
