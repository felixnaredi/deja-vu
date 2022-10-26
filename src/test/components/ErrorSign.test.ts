import ErrorSign from "@/components/ErrorSign.vue";
import { mount } from "@vue/test-utils";

describe("unit tests", () => {
  test("`ErrorSign` contains the expected slot", () => {
    const wrapper = mount(ErrorSign, {
      slots: {
        default: "<p>1330050845120928966</p>",
      },
    });
    expect(wrapper.find("p").text()).toBe("1330050845120928966");
  });

  test("`goHome` is triggered when button is clicked", async () => {
    const wrapper = mount(ErrorSign);
    wrapper.vm.goHome = vi.fn();
    await wrapper.find("button").trigger("click");
    expect(wrapper.vm.goHome).toBeCalled();
  });

  test("`goHome` sets `window.location.href` to `BASE_URL`", async () => {
    global.window = Object.create(window);
    Object.defineProperty(window, "location", { value: { href: "INITIAL" } });

    const wrapper = mount(ErrorSign);
    await wrapper.find("button").trigger("click");

    // TODO:
    //   Configure so that `import.meta.env.BASE_URL` does not show as an error in vscode.
    // @ts-ignore
    expect(window.location.href).toBe(import.meta.env.BASE_URL);
  });
});

describe("snapshots", () => {
  test("snapshot of empty slot", () => {
    const wrapper = mount(ErrorSign);
    expect(wrapper.html()).toMatchSnapshot();
  });

  test("snapshot of realistic usage of slot", () => {
    const wrapper = mount(ErrorSign, {
      slots: {
        default: "<p>This is a test. Do not panic</p>",
      },
    });
    expect(wrapper.html()).toMatchSnapshot();
  });
});
