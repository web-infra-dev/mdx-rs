import { compile as compileByMdxRs } from "../index.js";
import fs from "fs";
import { compile as compileByMdxJs } from "@mdx-js/mdx";
import Table from "cli-table";
import path from "path";
import { fileURLToPath } from "url";

const table = new Table({
  head: ["Name", "Time Spend"],
});

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const examplePath = path.resolve(__dirname, "./fixtures/example.md");
const content = fs.readFileSync(examplePath, "utf-8");

async function benchMdxjs() {
  await Promise.all(
    new Array(1000).fill(0).map(async (_, index) => {
      await compileByMdxJs(content);
    })
  );
}

async function benchMdxRs() {
  await Promise.all(
    new Array(1000).fill(0).map(async (_, index) => {
      await compileByMdxRs({
        value: content,
        filepath: "",
        development: true,
        root: "",
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
