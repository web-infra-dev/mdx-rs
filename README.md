# @modern-js/mdx-binding

This is a Node.js binding for MDX compliation of [Modern.js Doc](https://modernjs.dev/doc-tools) which is a modern documentation tool based on [Rspack](https://www.rspack.org/).

> Currently the repo is empty, but the source code will be open sourced soon.

## Credits

Thanks to [mdxjs-rs](https://github.com/wooorm/mdxjs-rs), the awesome Rust library authored by [wooorm](https://github.com/wooorm).We forked this library and customize it for Modern.js Doc, such as:

- Add Node.js binding so that we can use it in Node.js.
- Implement the logic of custom code block DOM struction
- Implement container grammer like `:::tip` in [@modern-js/remark-container](https://github.com/web-infra-dev/modern.js/tree/main/packages/toolkit/remark-container).
- ...

Also, thanks to [napi-rs](https://github.com/napi-rs/napi-rs), authored by [Brooooooklyn](https://github.com/Brooooooklyn), which is a great solution to help us build Node.js binding for Rust.
