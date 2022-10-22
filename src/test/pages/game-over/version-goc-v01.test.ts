/**
 * @vitest-environment jsdom
 */

import App from "@/pages/game-over/App.vue";
import { mount, flushPromises, VueWrapper } from "@vue/test-utils";
import { createTestingPinia } from "@pinia/testing";

function mountApp(href: string): VueWrapper<any> {
  return mount(App, {
    props: {
      href: href,
    },
    global: {
      plugins: [
        createTestingPinia({
          stubActions: false,
        }),
      ],
    },
  });
}

describe("load page/game-over.vue from valid an URL of version 'goc-v01'", () => {
  test("mount with a game run with`Top999WiktionaryFr`", async () => {
    let wrapper = mountApp(
      "https://felixnaredi.github.io/deja-vu/game-over/?version=goc-v01&checksum=1281838854530562919&data=eyJzZWVkIjoxNjY2MzM0NzIyNTM2LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE1LDU3LDU5XSwiZWxlbWVudF9jaGVja3N1bSI6MTU4MTM4MzY1OTQ4NDYxNTk5OTF9&unseen_set_id=Top999WiktionaryFr"
    );
    await flushPromises();
    expect(wrapper.vm.lives).toBe(0);
    expect(wrapper.vm.score).toBe(57);
    expect(wrapper.vm.errorMessage).toBe(undefined);
  });
  test("mount with a game run with `DictionaryFr`", async () => {
    let wrapper = mountApp(
      "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=8514807120037939996&data=eyJzZWVkIjoxNjY2NDU0NTYzMDU0LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzEzLDE2LDIyXSwiZWxlbWVudF9jaGVja3N1bSI6MzA0ODkyNTYyMjgxOTEzMzA0MX0%3D&unseen_set_id=DictionaryFr01"
    );
    await flushPromises();
    expect(wrapper.vm.lives).toBe(0);
    expect(wrapper.vm.score).toBe(20);
    expect(wrapper.vm.errorMessage).toBe(undefined);
  });
});

describe("load *page/game-over.vue* from an URL where either data or checksum has been modified to trigger `BadChecksum`", () => {
  test("mount with modified checksum", async () => {
    let wrapper = mountApp(
      "http://localhost:5173/game-over?version=goc-v01&checksum=3604969389934149523&data=eyJzZWVkIjoxNjY2NDU1NjkxMzAwLCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzIsNiwyMV0sImVsZW1lbnRfY2hlY2tzdW0iOjM4MTU5MzAyMTgwNTM2NDQxOTZ9&unseen_set_id=Top999WiktionaryFr"
    );
    await flushPromises();
    expect(wrapper.vm.errorMessage).toBeDefined();
    expect(wrapper.vm.lives).toBe(undefined);
    expect(wrapper.vm.score).toBe(undefined);
  });
});
