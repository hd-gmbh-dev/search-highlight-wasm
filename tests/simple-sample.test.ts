import { describe, it, expect } from "vitest";
import {
  highlight_ngram3
} from "../pkg/search_highlight_wasm";

describe("minimal text search highlight via wasm", async () => {
  it("should be highlighted to expected trigram", async () => {
    expect(highlight_ngram3("this is my test str", "this is ")).eq("<b>this is </b>my test str")
  });
  
  it("should return original string", async () => {
    expect(highlight_ngram3("this is", "this is my test str")).eq("this is")
  });
});