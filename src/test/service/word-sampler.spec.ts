import "mocha";
import { expect } from "chai";
import WordSampler from "../../service/word-sampler";

describe("WordSampler", () => {
  it("should never return the same element twice in a row from next", () => {
    const sampler = new WordSampler([1, 2], 0.5);
    let previous = sampler.next();
    for (let i = 0; i < 8; i++) {
      let current = sampler.next();
      expect(previous).to.not.equal(current);
      previous = current;
    }
  });

  it(
    "should return two unseen words to begin with and then only one of those " +
      "when `seenDistribution` is set to 1",
    () => {
      const sampler = new WordSampler([1, 2, 3], 1);
      const seen = new Set();

      seen.add(sampler.next());
      seen.add(sampler.next());
      expect(2).to.equal(seen.size);

      for (let i = 0; i < 8; i++) {
        seen.add(sampler.next());
        expect(2).to.equal(seen.size);
      }
    }
  );

  it("should only return seen words if unseen words runs out", () => {
    const sampler = new WordSampler([1, 2, 3], 0);
    const seen = new Set();

    for (let i = 0; i < 3; i++) {
      const x = sampler.next();
      expect(seen).to.not.include(x);
      seen.add(x);
    }
    for (let i = 0; i < 8; i++) {
      expect(seen).to.include(sampler.next());
    }
  });

  it("`current` should be the same as the last element generated by `next`", () => {
    const sampler = new WordSampler(
      Array.from(new Array(128)).map((_, index) => index),
      0.5
    );
    let previous = null;

    for (let i = 0; i < 128; i++) {
      expect(sampler.current).to.equal(previous);
      previous = sampler.next();
    }
  });
});
