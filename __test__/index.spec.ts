import { readFileSync } from "fs";
import { describe, test, expect } from "vitest";
import path from "path";
import prettier from "prettier";

import { compile } from "../index.js";

const formatHTML = (html: string) => prettier.format(html, { parser: "html" });

describe("compile", () => {
  test("should render container type correctly", async (t) => {
    let { code: result, html } = await compile({
      value: readFileSync(path.join(__dirname, "./container-type.md"), "utf8"),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
      defaultLang: "",
    });

    expect(formatHTML(html)).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container content correctly", async (t) => {
    let { code: result, html } = await compile({
      value: readFileSync(
        path.join(__dirname, "./container-content.md"),
        "utf8"
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
      defaultLang: "",
    });

    expect(formatHTML(html)).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container title correctly", async (t) => {
    let { code: result, html } = await compile({
      value: readFileSync(path.join(__dirname, "./container-title.md"), "utf8"),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
      defaultLang: "",
    });

    expect(formatHTML(html)).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });
});
