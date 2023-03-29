import { readFileSync } from "fs";
import { describe, test, expect } from "vitest";
import path from "path";

import { compile } from "../index.js";

describe("compile", () => {
  test("container grammer", async (t) => {
    let { code: result } = await compile({
      value: readFileSync(path.join(__dirname, "./container.md"), "utf8"),
      filepath: "xxx.mdx",
      development: true,
      root: "xxx",
      defaultLang: "",
    });

    expect(result).toMatchSnapshot();
  });
});
