/**
 * @vitest-environment jsdom
 */

import App from "@/pages/game-over/App.vue";
import ErrorSign from "@/components/ErrorSign.vue";
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

// TODO:
//   It would be better if the test also checks what kind of error is thrown but it is probably not
//   good to have it tied to a `string` literal that may change soon.

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

function expectDecodeError(wrapper: VueWrapper<any>) {
  const errorSign = wrapper.findComponent(ErrorSign);
  expect(errorSign).toBeTruthy();
  expect(wrapper.vm.errorMessage).toBeDefined();
  expect(wrapper.vm.errorMessage.trimEnd()).toBe(
    errorSign.find("p").text().trimEnd()
  );
  expect(wrapper.vm.lives).toBe(undefined);
  expect(wrapper.vm.score).toBe(undefined);
}

describe("Passing bad URL:s expecting error", () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {});

  afterEach(() => {
    expect(error).toBeCalled();
    error?.mockReset();
  });

  describe("load *page/game-over.vue* from an URL where either data or checksum has been modified to trigger `BadChecksum`", () => {
    test("mount with modified checksum", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=3604969389934149523&data=eyJzZWVkIjoxNjY2NDU1NjkxMzAwLCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzIsNiwyMV0sImVsZW1lbnRfY2hlY2tzdW0iOjM4MTU5MzAyMTgwNTM2NDQxOTZ9&unseen_set_id=Top999WiktionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    test("mount with modified data", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=11173297498535147619&data=eyJzZWVkIjoxNjY2NDY1NjkxODAwLCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzEzLDM3LDY5XSwiZWxlbWVudF9jaGVja3N1bSI6MTMzMTg2MDY1ODc5NTA5ODYyODZ9Cg%3D%3D&unseen_set_id=Top999WiktionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
  });

  describe("URL:s missing fields needed for decoding.", () => {
    test("missing every field", async () => {
      let wrapper = mountApp("http://felixnaredi.github.io/deja-vu/game-over");
      await flushPromises();
      expectDecodeError(wrapper);
    });
    test("missing 'version' field", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?checksum=4343440950591163727&data=eyJzZWVkIjoxNjY2NDY3ODA5NDY1LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE4LDI5LDMwXSwiZWxlbWVudF9jaGVja3N1bSI6NzcyMjk2ODE4NDkxNTk1MDY5fQ%3D%3D&unseen_set_id=Top999WiktionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    test("missing 'checksum' field", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&data=eyJzZWVkIjoxNjY2NDY3ODA5NDY1LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE4LDI5LDMwXSwiZWxlbWVudF9jaGVja3N1bSI6NzcyMjk2ODE4NDkxNTk1MDY5fQ%3D%3D&unseen_set_id=Top999WiktionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    test("missing 'data' field", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=4343440950591163727&unseen_set_id=Top999WiktionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    test("missing 'unseen_set_id' field", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=4343440950591163727&data=eyJzZWVkIjoxNjY2NDY3ODA5NDY1LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE4LDI5LDMwXSwiZWxlbWVudF9jaGVja3N1bSI6NzcyMjk2ODE4NDkxNTk1MDY5fQ%3D%3D"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    /*
    test("missing 'unseen_set_id' field with game run with 'DictionaryFr'", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=1405434506133277248&data=eyJzZWVkIjoxNjY2NDY4MDc1MTgwLCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzcsMTksMjNdLCJlbGVtZW50X2NoZWNrc3VtIjoxNDYzODY0MDc1ODk2MTU5NDIwN30%3D"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
    */
  });

  describe("decode with wrong 'unseen_set_id'", () => {
    test("Decode game run with 'Top999WiktionaryFr' but with 'DictionaryFr' in URL", async () => {
      let wrapper = mountApp(
        "http://felixnaredi.github.io/deja-vu/game-over?version=goc-v01&checksum=9563548670754739164&data=eyJzZWVkIjoxNjY2NDY4Mzg1NTA4LCJzZWVuX3RocmVzaG9sZCI6NDAwMDAwMDAwLCJpbmNvcnJlY3RfY29tbWl0cyI6WzE3LDIxLDM1XSwiZWxlbWVudF9jaGVja3N1bSI6MTQ0NjAxOTA5NDE5NTc3MjMzMzV9&unseen_set_id=DictionaryFr"
      );
      await flushPromises();
      expectDecodeError(wrapper);
    });
  });
});
