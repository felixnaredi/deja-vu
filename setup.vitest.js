import createFetchMock from "vitest-fetch-mock";
import { vi } from "vitest";
import _obj0 from "./public/dictionary/fr01/words.json";
import _obj1 from "./public/dictionary/top999-wiktionary-fr/words.json";

const dictionary = {
  fr01: _obj0,
  top999WiktionaryFr: Object.values(_obj1),
};

fetch = createFetchMock(vi);
fetch.mockResponse((request) => {
  if (/\/dictionary\/fr01\/words.json$/.test(request.url)) {
    return new Response(JSON.stringify(dictionary.fr01));
  }
  if (/\/dictionary\/top999-wiktionary-fr\/words.json$/.test(request.url)) {
    return new Response(JSON.stringify(dictionary.top999WiktionaryFr));
  }
  return new Response("Not found", { status: 404 });
});
fetch.enableMocks();
