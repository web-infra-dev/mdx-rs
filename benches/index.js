const { compile } = require("../index.js");
const fs = require("fs");

const start = Date.now();
const content = fs.readFileSync("./fixtures/example.md", "utf-8");

async function bench() {
  await Promise.all(
    new Array(700).fill(0).map(async (_, index) => {
      await compile(content, `${index}.md`);
    })
  );
  console.log(`time: ${Date.now() - start}ms`);
}

bench();
