/**
 * @vitest-environment jsdom
 */

import App from "@/pages/game-over/App.vue";
import { createPinia, setActivePinia } from "pinia";
import { mount, flushPromises } from "@vue/test-utils";
import { createTestingPinia } from "@pinia/testing";
import _obj0 from "../../../public/dictionary/fr01/words.json";
import _obj1 from "../../../public/dictionary/top999-wiktionary-fr/words.json";

const dictionary = {
  fr01: _obj0,
  top999WiktionaryFr: Object.values(_obj1),
};

test("mount App with valid URL", async () => {
  expect(App).toBeTruthy();

  setActivePinia(createPinia());
  fetch.mockResponse(JSON.stringify(dictionary.top999WiktionaryFr));

  const pinia = createTestingPinia();

  let wrapper = mount(App, {
    props: {
      href: "https://felixnaredi.github.io/deja-vu/game-over/?version=goc-v01&checksum=1281838854530562919&data=eyJzZWVkIjoxNjY2MzM0NzIyNTM2LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE1LDU3LDU5XSwiZWxlbWVudF9jaGVja3N1bSI6MTU4MTM4MzY1OTQ4NDYxNTk5OTF9&unseen_set_id=Top999WiktionaryFr",
    },
    global: {
      plugins: [
        createTestingPinia({
          stubActions: false,
        }),
      ],
    },
  });

  await flushPromises();
  expect(wrapper.vm.lives).toBe(0);
  expect(wrapper.vm.score).toBe(57);
  expect(wrapper.vm.errorMessage).toBe(undefined);
});
