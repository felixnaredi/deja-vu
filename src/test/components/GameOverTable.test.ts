import GameOverTable from "@/components/GameOverTable.vue";
import GameOverTableRow from "@/components/GameOverTableRow.vue";
import { mount, shallowMount, VueWrapper } from "@vue/test-utils";
import { createTestingPinia } from "@pinia/testing";
import { useGameOverStore } from "@/store/game-over";
import Commit from "@/interfaces/Commit";
import { generateCommit, Seen, Unseen } from "@/test/commit-mock";
import CrossSymbol from "@/assets/CrossSymbol.vue";
import TickSymbol from "@/assets/TickSymbol.vue";
import EyeSymbol from "@/assets/EyeSymbol.vue";
import { setActivePinia } from "pinia";

function createWrapper(
  mountFunction: typeof shallowMount | typeof mount,
  commits: undefined | Array<Commit>
): VueWrapper<any> {
  const pinia = createTestingPinia();
  setActivePinia(pinia);
  const gameOverStore = useGameOverStore(pinia);

  // `createTestingPinia` makes `commits` into a writable property
  // @ts-ignore
  gameOverStore.commits = commits;

  return mountFunction(GameOverTable);
}

function shallowGameOverTable(
  commits: undefined | Array<Commit>
): VueWrapper<any> {
  return createWrapper(shallowMount, commits);
}

function mountGameOverTable(
  commits: undefined | Array<Commit>
): VueWrapper<any> {
  return createWrapper(mount, commits);
}

describe("unit tests", () => {
  test("no rows, only header, when commit list is undefined", () => {
    // TODO:
    //   Muting this output. Could be of interest to solve why the warning in triggered and prevent
    //   it.
    // ```
    // stderr | src/test/components/GameOverTable.test.ts > unit tests > no rows, only header, when commit list is undefined
    // [Vue warn]: injection "Symbol(pinia)" not found.
    //   at <GameOverTable ref="VTU_COMPONENT" >
    //   at <VTUROOT>
    // ```
    vi.spyOn(console, "warn").mockImplementation(() => {});

    const wrapper = shallowGameOverTable(undefined);
    expect(wrapper.html()).toContain("déjà vu");
    expect(wrapper.html()).toContain("nouveau");
    expect(wrapper.findComponent(GameOverTableRow).exists()).not.toBeTruthy();
  });

  test("no rows, only header, when commit list is empty", () => {
    // TODO:
    //   Muting this output. Could be of interest to solve why the warning in triggered and prevent
    //   it.
    // ```
    // stderr | src/test/components/GameOverTable.test.ts > unit tests > no rows, only header, when commit list is empty
    // stderr | src/test/components/GameOverTable.test.ts > unit tests > no rows, only header, when commit list is empty
    // [Vue warn]: injection "Symbol(pinia)" not found.
    //   at <GameOverTable ref="VTU_COMPONENT" >
    //   at <VTUROOT>
    //
    // [Vue warn]: injection "Symbol(pinia)" not found.
    //   at <GameOverTable ref="VTU_COMPONENT" >
    //   at <VTUROOT>
    // ```
    vi.spyOn(console, "warn").mockImplementation(() => {});

    const wrapper = shallowGameOverTable([]);
    expect(wrapper.html()).toContain("déjà vu");
    expect(wrapper.html()).toContain("nouveau");
    expect(wrapper.findComponent(GameOverTableRow).exists()).not.toBeTruthy();
  });

  test("no rows, only header, when commit list is empty", () => {
    const wrapper = mountGameOverTable([
      generateCommit({
        seed: 1818439690026315,
        actual: new Seen(),
        guess: new Seen(),
      }),
      generateCommit({
        seed: 881250011915912,
        actual: new Unseen(),
        guess: new Seen(),
      }),
      generateCommit({
        seed: 228821152228297,
        actual: new Seen(),
        guess: new Unseen(),
      }),
      generateCommit({
        seed: 8034100225224008,
        actual: new Unseen(),
        guess: new Unseen(),
      }),
    ]);
    expect(wrapper.html()).toContain("déjà vu");
    expect(wrapper.html()).toContain("nouveau");
    expect(wrapper.findAllComponents(GameOverTableRow).length).toBe(4);
    expect(wrapper.findAllComponents(CrossSymbol).length).toBe(2);
    expect(wrapper.findAllComponents(TickSymbol).length).toBe(2);
    expect(wrapper.findAllComponents(EyeSymbol).length).toBe(2);
  });
});

describe("snapshots", () => {
  test("snapshot of `GameOverTable` with `commits` set to `undefined`", () => {
    const wrapper = shallowGameOverTable(undefined);
    expect(wrapper.html()).toMatchSnapshot();
  });
  test("snapshot of `GameOverTable` with `commits` set to empty list", () => {
    const wrapper = shallowGameOverTable([]);
    expect(wrapper.html()).toMatchSnapshot();
  });
  test("snapshot of `GameOverTable` with `commits` some generated `Commit`s", () => {
    const wrapper = mountGameOverTable(
      [
        1835952554073773, 76584337047733, 158407870645769, 3806636883036028,
        999837377973796, 1181176957149847, 147709579785058, 1645148002455520,
        1571701253944690, 566009344100606, 370914558206118,
      ].map((seed) => generateCommit({ seed }))
    );
    expect(wrapper.html()).toMatchSnapshot();
  });
});
