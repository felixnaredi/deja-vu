import UnseenSetID from "@/service/unseen-set-id";
import { KSINK, UnseenSetIDPrimitive } from "deja-vu-wasm";

// Other valus of `UnseenSetID` are not accessible.
let ids = [UnseenSetID.DictionaryFr01, UnseenSetID.Top999WiktionaryFr];

describe("unit tests", () => {
  test("correct word list is fetched", () => {
    for (const id of ids) {
      id.words.then((words) => {
        expect(
          words
            .slice(1)
            .reduce(
              (c, x) => KSINK.hashString(c, x),
              KSINK.hashString(BigInt(25609135034806), words[0])
            )
        ).toMatchSnapshot();
      });
    }
  });
  test("two `UnseenSetID`s with the same value are equal even if they are not the same reference", () => {
    expect(ids).toContainEqual(UnseenSetID.DictionaryFr01);
    expect(ids).toContainEqual(UnseenSetID.Top999WiktionaryFr);
  });
  test("two `UnseenSetID`s of different values are not equal", () => {
    let _ids = new Array(...ids);
    while (_ids.length > 1) {
      const id = _ids.pop();
      expect(_ids).not.toContainEqual(id);
    }
  });
  test("no two diffeernt `UnseenSetID`s has the same `UnseenSetIDPrimitive`", () => {
    let primitives = ids.map((x) => x.primitive);
    while (primitives.length > 1) {
      const p = primitives.pop();
      expect(primitives).not.toContainEqual(p);
    }
  });
});
