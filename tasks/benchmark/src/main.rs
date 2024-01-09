extern crate criterion;
extern crate mdx_rs;
extern crate pico_args;

use criterion::{BenchmarkId, Criterion, Throughput};
use mdx_rs::{compile, CompileOptions};
use pico_args::Arguments;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
  let mut args = Arguments::from_env();
  let baseline: Option<String> = args.opt_value_from_str("--save-baseline").unwrap();
  let mut criterion = Criterion::default().without_plots();
  if let Some(baseline) = baseline {
    criterion = criterion.save_baseline(baseline.to_string());
  }
  // Define the path to the file
  let path: &str = "benches/fixtures/example.md";
  // Open the file in read-only mode
  let mut file = File::open(&path).expect("Could not open file");

  // Create a new String to hold the file contents
  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .expect("Could not read file");

  let mut group = criterion.benchmark_group("mdx_rs");
  group.throughput(Throughput::Bytes(contents.len() as u64));
  group.bench_with_input(
    BenchmarkId::from_parameter("example.md"),
    &contents,
    |b, source_text| {
      b.iter_with_large_drop(|| {
        compile(CompileOptions {
          value: source_text.to_string(),
          development: false,
          ..Default::default()
        });
      })
    },
  );
  group.finish();
  drop(criterion);
}
