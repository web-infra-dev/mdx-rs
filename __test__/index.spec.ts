import { readFileSync } from "node:fs";
import path from "node:path";
import { createSnapshotSerializer } from "path-serializer";
// @ts-ignore TODO: add types for pretter
import prettier from "prettier";
import { describe, expect, test } from "vitest";

import { type CompileOptions, compile } from "../index.js";

expect.addSnapshotSerializer(
  createSnapshotSerializer({
    features: {
      escapeDoubleQuotes: false,
    },
  }),
);

const formatHTML = (html: string) => {
  return prettier.format(html, { parser: "html" });
};

const formatResult = async (result: string): Promise<string> => {
  // For win ci
  return prettier.format(result, { parser: "babel-ts" });
};

const testCompile = async (options: CompileOptions) => {
  const { code: result, html } = await compile(options);

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
  test("should render container type correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(path.join(__dirname, "./container-type.md"), "utf8"),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container type with space correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-type-with-space.md"),
        "utf8",
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container content correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-content.md"),
        "utf8",
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container title in mdx correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(
        path.join(__dirname, "./container-title.mdx"),
        "utf8",
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render container title in md correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(path.join(__dirname, "./container-title.md"), "utf8"),
      filepath: "xxx.md",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });

  test("should render github alerts correctly", async () => {
    const { code: result, html } = await testCompile({
      value: readFileSync(
          path.join(__dirname, "./github-alert-syntax.md"),
          "utf8",
      ),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
    });

    expect(html).toMatchSnapshot();
    expect(result).toMatchSnapshot();
  });
});
