use mdx_rs::{self, CompileResult};
// mod thread_safe_function;

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AsyncTask, Result, Task},
  JsObject,
};

#[napi(object)]
pub struct Output {
  pub code: String,
  pub links: Vec<String>,
}

impl From<CompileResult> for Output {
  fn from(res: CompileResult) -> Self {
    Self {
      code: res.code,
      links: res.links,
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
    Ok(obj)
  }
}

pub struct Compiler {
  value: String,
  filepath: String,
  development: bool,
  root: String,
  default_lang: String,
}

impl Compiler {
  pub fn new(
    value: String,
    filepath: String,
    development: bool,
    root: String,
    default_lang: String,
  ) -> Self {
    Self {
      value,
      filepath,
      development,
      root,
      default_lang,
    }
  }

  fn compile(&mut self) -> CompileResult {
    mdx_rs::compile(
      &self.value,
      &self.filepath,
      self.development,
      &self.root,
      &self.default_lang,
    )
  }
}

/// Turn MDX into JavaScript.
#[napi(ts_return_type = "Promise<Output>")]
pub fn compile(
  value: String,
  filepath: String,
  development: bool,
  root: String,
  default_lang: String,
) -> AsyncTask<Compiler> {
  AsyncTask::new(Compiler::new(
    value,
    filepath,
    development,
    root,
    default_lang,
  ))
}

#[napi]
pub fn compile_sync(
  value: String,
  filepath: String,
  development: bool,
  root: String,
  default_lang: String,
) -> Output {
  let mut compiler = Compiler::new(value, filepath, development, root, default_lang);
  compiler.compile().into()
}
