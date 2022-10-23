/**
 * @vitest-environment jsdom
 */

import GradientButton from "@/components/GradientButton.vue";
import { VueWrapper, mount } from "@vue/test-utils";

const TEXT = "GHCi, version 9.4.2";

function mountGradientButton(): VueWrapper<any> {
  return mount(GradientButton, {
    slots: {
      default: `<p>${TEXT}</p>`,
    },
  });
}

test("check that the button emits 'click' when clicked", async () => {
  const wrapper = mountGradientButton();
  wrapper.find("button").trigger("click");
  await wrapper.vm.$nextTick();
  expect(wrapper.emitted().click).toBeTruthy();
});

test("check that the button slot contains what is expected", () => {
  const wrapper = mountGradientButton();
  expect(wrapper.find("button").find("p").text()).toBe(TEXT);
});
