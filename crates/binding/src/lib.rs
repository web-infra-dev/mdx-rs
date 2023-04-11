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
}

#[napi(object)]
pub struct CompileOptions {
  pub value: String,
  pub filepath: String,
  pub development: bool,
  pub root: String,
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
    Ok(obj)
  }
}

pub struct Compiler {
  value: String,
  filepath: String,
  development: bool,
  root: String,
}

impl Compiler {
  pub fn new(value: String, filepath: String, development: bool, root: String) -> Self {
    Self {
      value,
      filepath,
      development,
      root,
    }
  }

  fn compile(&mut self) -> CompileResult {
    mdx_rs::compile(&self.value, &self.filepath, self.development, &self.root)
  }
}

/// Turn MDX into JavaScript.
#[napi(ts_return_type = "Promise<Output>")]
pub fn compile(options: CompileOptions) -> AsyncTask<Compiler> {
  let CompileOptions {
    value,
    filepath,
    development,
    root,
  } = options;
  AsyncTask::new(Compiler::new(value, filepath, development, root))
}

#[napi]
pub fn compile_sync(options: CompileOptions) -> Output {
  let CompileOptions {
    value,
    filepath,
    development,
    root,
  } = options;
  let mut compiler = Compiler::new(value, filepath, development, root);
  compiler.compile().into()
}
