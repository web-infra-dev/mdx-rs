import { readFileSync } from "fs";
import { describe, test, expect } from "vitest";
import path from "path";

import { compile } from "../index.js";

describe("compile", () => {
  test("container grammer", async (t) => {
    let result = await compile(
      readFileSync(path.join(__dirname, "./container.md"), "utf8"),
      "xxx.mdx"
    );

    expect(result).toMatchSnapshot();
  });
});
