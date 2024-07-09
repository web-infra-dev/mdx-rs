import { readFileSync } from "fs";
import { describe, test, expect } from "vitest";
import path from "path";
// @ts-ignore TODO: add types for pretter
import prettier from "prettier";

import { compile, type CompileOptions } from "../index.js";


const formatHTML = (html: string) => {
  return prettier.format(html, { parser: "html" })
};

const formatResult = async (result: string): Promise<string> => {
  // For win ci
  const replacedResult = result.replaceAll('\\r\\n', '<LF>').replaceAll('\\n', '<LF>');
  return prettier.format(replacedResult, { parser: "babel-ts" })
};

const testCompile = async (options: CompileOptions) => {
  let { code: result, html } = await compile(options);

  const [formattedResult, formattedHtml] = await Promise.all([
    formatResult(result),
    formatHTML(html),
  ]);

  return {
    code: formattedResult,
    html: formattedHtml,
  };
};



describe("compile", () => {
  test("should render container type correctly", async (t) => {
    let { code: result, html } = await testCompile({
      value: readFileSync(path.join(__dirname, "./container-type.md"), "utf8"),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container type with space correctly", async (t) => {
    let { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-type-with-space.md"),
        "utf8"
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container content correctly", async (t) => {
    let { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-content.md"),
        "utf8"
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container title in mdx correctly", async (t) => {
    let { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-title.mdx"),
        "utf8"
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container title in md correctly", async (t) => {
    let { code: result, html } = await testCompile({
      value: readFileSync(path.join(__dirname, "./container-title.md"), "utf8"),
      filepath: "xxx.md",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });
});
