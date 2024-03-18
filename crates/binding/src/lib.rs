#[cfg(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

use mdx_plugin_toc::TocItem;
use mdx_rs::{self, CompileResult};

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Result, Task},
  JsObject,
};

#[napi(object)]
pub struct Toc {
  pub text: String,
  pub id: String,
  pub depth: u8,
}

#[napi(object)]
pub struct Output {
  pub code: String,
  pub links: Vec<String>,
  pub html: String,
  pub title: String,
  pub toc: Vec<Toc>,
  pub frontmatter: String,
}

#[napi(object)]
pub struct CompileOptions {
  pub value: String,
  pub filepath: String,
  pub development: bool,
  pub root: String,
  pub jsx: Option<bool>,
}

impl From<TocItem> for Toc {
  fn from(item: TocItem) -> Self {
    Self {
      text: item.text,
      id: item.id,
      depth: item.depth,
    }
  }
}

impl From<CompileResult> for Output {
  fn from(res: CompileResult) -> Self {
    Self {
      code: res.code,
      links: res.links,
      html: res.html,
      title: res.title,
      toc: res.toc.into_iter().map(|item| item.into()).collect(),
      frontmatter: res.frontmatter,
    }
  }
}

impl Task for Compiler {
  type Output = CompileResult;
  type JsValue = JsObject;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(self.compile())
  }

  fn resolve(&mut self, env: napi::Env, output: CompileResult) -> Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", output.code)?;
    obj.set_named_property("links", output.links)?;
    obj.set_named_property("html", output.html)?;
    obj.set_named_property("title", output.title)?;
    obj.set_named_property(
      "toc",
      output
        .toc
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<Toc>>(),
    )?;
    obj.set_named_property("frontmatter", output.frontmatter)?;
    Ok(obj)
  }
}

pub struct Compiler {
  options: CompileOptions,
}

impl Compiler {
  pub fn new(options: CompileOptions) -> Self {
    Self { options }
  }

  fn compile(&mut self) -> CompileResult {
    mdx_rs::compile(mdx_rs::CompileOptions {
      value: self.options.value.clone(),
      filepath: self.options.filepath.clone(),
      development: self.options.development,
      root: self.options.root.clone(),
      jsx: self.options.jsx.unwrap_or(true),
    })
  }
}

/// Turn MDX into JavaScript.
#[napi(ts_return_type = "Promise<Output>")]
pub fn compile(options: CompileOptions) -> AsyncTask<Compiler> {
  AsyncTask::new(Compiler::new(options))
}

#[napi]
pub fn compile_sync(options: CompileOptions) -> Output {
  let mut compiler = Compiler::new(options);
  compiler.compile().into()
}
