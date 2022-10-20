import { UnseenSetIDPrimitive } from "deja-vu-wasm";
import path from "@/service/path";

/**
 * Accessor for an unseen set.
 */
class UnseenSetID {
  private _primitive: UnseenSetIDPrimitive;

  /**
   * Initializes a `UnseenSetID` from an `UnseenSetIDPrimitive`.
   *
   * @param primitive The `UnseenSetIDPrimitive` to init with.
   */
  public constructor(primitive: UnseenSetIDPrimitive) {
    this._primitive = Number(primitive);
  }

  /**
   * The label the `UnseenSetID` should be display in a drop down menu.
   */
  public get menuItemLabel(): string {
    switch (this._primitive) {
      case UnseenSetIDPrimitive.Unspecified:
        return "Unspecified";
      case UnseenSetIDPrimitive.DictionaryFr01:
        return "🇫🇷 liste français";
      case UnseenSetIDPrimitive.Top999WiktionaryFr:
        return "🇫🇷 999 mots les plus courants";
    }
    return "<unknown>";
  }

  /**
   * Words in the `UnseenSet`.
   */
  public get words(): Promise<string[]> {
    switch (this._primitive) {
      case UnseenSetIDPrimitive.DictionaryFr01:
        return fetch(
<<<<<<< HEAD
          path(process.env.BASE_URL!, "dictionary", "fr01", "words.json")
=======
          path(import.meta.env.BASE_URL!, "dictionary", "fr01", "words.json")
>>>>>>> main
        )
          .then((response) => response.json())
          .then((words) => words);
      case UnseenSetIDPrimitive.Top999WiktionaryFr:
        return fetch(
          path(
<<<<<<< HEAD
            process.env.BASE_URL!,
=======
            import.meta.env.BASE_URL!,
>>>>>>> main
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

  /**
   * The inner primitive.
   */
  public get primitive(): UnseenSetIDPrimitive {
    return this._primitive;
  }

  /**
   * French...
   */
  public static get DictionaryFr01(): UnseenSetID {
    return new UnseenSetID(UnseenSetIDPrimitive.DictionaryFr01);
  }

  /**
   * Top 999 most common french words according to Wiktionary.
   */
  public static get Top999WiktionaryFr(): UnseenSetID {
    return new UnseenSetID(UnseenSetIDPrimitive.Top999WiktionaryFr);
  }
}

export default UnseenSetID;
