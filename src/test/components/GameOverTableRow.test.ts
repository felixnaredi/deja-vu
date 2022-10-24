import { generateCommit, Seen, Unseen } from "@/test/commit-mock";
import { mount, shallowMount } from "@vue/test-utils";
import GameOverTableRow from "@/components/GameOverTableRow.vue";
import EyeSymbol from "@/assets/EyeSymbol.vue";
import TickSymbol from "@/assets/TickSymbol.vue";
import CrossSymbol from "@/assets/CrossSymbol.vue";
import SeenUnseen from "@/interfaces/SeenUnseen";

let seed = 1406960267;

beforeEach(() => {
  seed += 1;
});

describe("unit tests", () => {
  test("seen commits renders an `EyeSymbol`, unseen does not", () => {
    for (let i = 0; i < 16; i++) {
      const commit = generateCommit({ seed: seed + i });
      const wrapper = shallowMount(GameOverTableRow, { props: { commit } });

      if (commit.actual().isSeen()) {
        expect(wrapper.findComponent(EyeSymbol).exists()).toBeTruthy();
      } else {
        expect(wrapper.findComponent(EyeSymbol).exists()).not.toBeTruthy();
      }
    }
    seed += 1;
  });
  describe("'TickSymbol' and 'CrossSymbol' renders as they should", () => {
    function check(actual: SeenUnseen, guess: SeenUnseen) {
      const commit = generateCommit({
        actual,
        guess,
        seed,
      });
      const wrapper = shallowMount(GameOverTableRow, { props: { commit } });

      if (commit.correct()) {
        expect(wrapper.findComponent(TickSymbol).exists()).toBeTruthy();
        expect(wrapper.findComponent(CrossSymbol).exists()).not.toBeTruthy();
      } else {
        expect(wrapper.findComponent(TickSymbol).exists()).not.toBeTruthy();
        expect(wrapper.findComponent(CrossSymbol).exists()).toBeTruthy();
      }
    }

    test("guessing 'unseen' correct renders a `TickSymbol` but no `CrossSymbol`", () => {
      check(new Unseen(), new Unseen());
    });
    test("guessing 'seen' correct renders a `TickSymbol` but no `CrossSymbol`", () => {
      check(new Seen(), new Seen());
    });
    test("guessing 'unseen' incorrect renders a `CrossSymbol` but no `TickSymbol`", () => {
      check(new Unseen(), new Seen());
    });
    test("guessing 'seen' incorrect renders a `CrossSymbol` but no `TickSymbol`", () => {
      check(new Seen(), new Unseen());
    });
  });
});
