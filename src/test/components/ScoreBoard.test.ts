import ScoreBoard from "@/components/ScoreBoard.vue";
import { mount } from "@vue/test-utils";

describe("unit tests", () => {
  test("empty render accordingly", () => {
    const wrapper = mount(ScoreBoard);
    const columns = wrapper.find("div").findAll("div");
    for (const { i, label } of [
      { i: 0, label: "score:" },
      { i: 2, label: "des vies:" },
    ]) {
      const spans = columns[i].findAll("span");
      expect(spans[0].text()).toBe(label);
      expect(spans[1].text()).toBe("");
    }
  });
  test("renders accordingly when score and lives prop changes", async () => {
    const wrapper = mount(ScoreBoard);
    for (const props of [
      { score: 0, lives: 0 },
      { score: 1, lives: 3 },
      { score: 7312, lives: -8312 },
      { score: -62, lives: 81 },
    ]) {
      await wrapper.setProps(props);
      const { score, lives } = props;
      const columns = wrapper.find("div").findAll("div");
      expect(columns[0].findAll("span")[1].text()).toBe(String(score));
      expect(columns[2].findAll("span")[1].text()).toBe(String(lives));
    }
  });
});

describe("snapshots", () => {
  test("empty", () => {
    expect(mount(ScoreBoard).html()).toMatchSnapshot();
  });
  test("score and lives prop changes", async () => {
    const wrapper = mount(ScoreBoard);
    for (const props of [
      { score: 0, lives: 0 },
      { score: 1, lives: 3 },
      { score: 7312, lives: -8312 },
      { score: -62, lives: 81 },
    ]) {
      await wrapper.setProps(props);
      expect(wrapper.html()).toMatchSnapshot();
    }
  });
});
