import { UnseenSetIDPrimitive } from "../../dist/wasm";
import path from "@/service/path";

class UnseenSetID {
  private id: UnseenSetIDPrimitive;

  private constructor(id: UnseenSetIDPrimitive) {
    this.id = id;
  }

  public get menuItemLabel(): string {
    switch (this.id) {
      case UnseenSetIDPrimitive.Unspecified:
        return "Unspecified";
      case UnseenSetIDPrimitive.DictionaryFr01:
        return "🇫🇷 liste français";
      case UnseenSetIDPrimitive.Top999WiktionaryFr:
        return "🇫🇷 999 mots les plus courants";
    }
    return "<unknown>";
  }

  public get words(): Promise<string[]> {
    switch (this.id) {
      case UnseenSetIDPrimitive.DictionaryFr01:
        return fetch(
          path(process.env.BASE_URL, "dictionary", "fr01", "words.json")
        )
          .then((response) => response.json())
          .then((words) => words);
      case UnseenSetIDPrimitive.Top999WiktionaryFr:
        return fetch(
          path(
            process.env.BASE_URL,
            "dictionary",
            "top999-wiktionary-fr",
            "words.json"
          )
        )
          .then((response) => response.json())
          .then((words) => Object.values(words));
    }
    return (async () => [])();
  }

  public static DictionaryFr01(): UnseenSetID {
    return new UnseenSetID(UnseenSetIDPrimitive.DictionaryFr01);
  }

  public static Top999WiktionaryFr(): UnseenSetID {
    return new UnseenSetID(UnseenSetIDPrimitive.Top999WiktionaryFr);
  }
}

export default UnseenSetID;
