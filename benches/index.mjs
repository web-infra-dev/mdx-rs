import { compile as compileByMdxRs } from "../index.js";
import fs from "fs";
import { compile as compileByMdxJs } from "@mdx-js/mdx";
import Table from "cli-table";
import path from "path";

const table = new Table({
  head: ["Name", "Time Spend"],
});

// Esm 环境中实现 __dirname
const __dirname = path.dirname(new URL(import.meta.url).pathname);

const examplePath = path.resolve(__dirname, "./fixtures/example.md");
const content = fs.readFileSync(examplePath, "utf-8");

async function benchMdxjs() {
  await Promise.all(
    new Array(800).fill(0).map(async (_, index) => {
      await compileByMdxJs(content);
    })
  );
}

async function benchMdxRs() {
  await Promise.all(
    new Array(800).fill(0).map(async (_, index) => {
      await compileByMdxRs({
        value: content,
        filepath: examplePath,
        development: true,
        root: "",
        defaultLang: "",
      });
    })
  );
}

async function bench() {
  let mdxRsTime = 0;
  let mdxJsTime = 0;

  let start = Date.now();
  await benchMdxjs();
  mdxJsTime = Date.now() - start;

  start = Date.now();
  await benchMdxRs();
  mdxRsTime = Date.now() - start;

  table.push(["mdx-js", `${mdxJsTime.toLocaleString()}ms`]);
  table.push(["mdx-rs", `${mdxRsTime.toLocaleString()}ms`]);

  console.log(table.toString());
}

bench();
