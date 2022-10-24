import { generateCommit } from "@/test/commit-mock";

test("temp0", () => {
  console.log(generateCommit({ seed: BigInt(100) }).element());
});

test("temp1", () => {
  console.log(
    generateCommit({ seed: BigInt(100) })
      .actual()
      .isSeen()
  );
});
