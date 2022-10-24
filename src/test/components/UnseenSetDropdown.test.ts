import UnseenSetDropdown from "@/components/UnseenSetDropdown.vue";
import UnseenSetID from "@/service/unseen-set-id";
import { mount } from "@vue/test-utils";

const unseenSetIDs = [
  UnseenSetID.DictionaryFr01,
  UnseenSetID.Top999WiktionaryFr,
];

let wrapper = mount(UnseenSetDropdown);
beforeEach(() => {
  wrapper = mount(UnseenSetDropdown);
});

describe("unit tests", () => {
  test("UnseenSetDropdown has the expected data", () => {
    expect(wrapper.vm.unseenSetIDs.length).toBe(unseenSetIDs.length);
    for (const id of unseenSetIDs) {
      expect(wrapper.vm.unseenSetIDs).toContainEqual(id);
    }
  });
  test("UnseenSetDropdown contains expected menu items", () => {
    const menuItemLabels = unseenSetIDs.map((id) => id.menuItemLabel);
    for (const option of wrapper.findAll("option")) {
      expect(menuItemLabels).toContain(option.text());
    }
  });
  test("'change' is emitted when dropdown menu is changed", () => {
    wrapper.find("select").trigger("change");
    wrapper.vm.$nextTick();
    expect(wrapper.emitted().change).toBeTruthy();
  });
  test("disabled is false by default", () => {
    expect(wrapper.props("disabled")).not.toBeTruthy();
  });
  test("selected element affects the UnseenSetDropdown", () => {
    const ws = unseenSetIDs.map((id) =>
      mount(UnseenSetDropdown, { props: { selected: id.primitive } })
    );
    while (ws.length > 1) {
      expect(ws).toContain(ws[ws.length - 1]);
      const w = ws.pop();
      expect(ws).not.toContain(w);
    }
  });
});

describe("snapshot", () => {
  test("snapshot matches with default UnseenSetDropdown", () => {
    expect(wrapper.html()).toMatchSnapshot();
  });
});
