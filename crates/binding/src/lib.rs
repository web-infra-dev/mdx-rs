use mdx_rs;

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::{AsyncTask, Error, Result, Status, Task};

impl Task for Compiler {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<String> {
    self
      .compile()
      .map_err(|err| Error::new(Status::GenericFailure, err))
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<String> {
    Ok(output)
  }
}

pub struct Compiler {
  value: String,
  filepath: String,
}

impl Compiler {
  pub fn new(value: String, filepath: String) -> Self {
    Self { value, filepath }
  }

  fn compile(&mut self) -> Result<String, String> {
    Ok(mdx_rs::compile(&self.value, &self.filepath))
  }
}

/// Turn MDX into JavaScript.
#[napi]
pub fn compile(value: String, filepath: String) -> AsyncTask<Compiler> {
  AsyncTask::new(Compiler::new(value, filepath))
}
