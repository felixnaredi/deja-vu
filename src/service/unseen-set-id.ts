import { UnseenSetIDPrimitive } from "deja-vu-wasm";

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
      case UnseenSetIDPrimitive.Unspecified:
        throw "`UnseedID.Unspecified` is not allowed to be used outside of testing";
      case UnseenSetIDPrimitive.DictionaryFr01:
        return fetch(new URL("/dictionary/fr01/words.json", import.meta.url))
          .then((response) => response.json())
          .then((words) => words);
      case UnseenSetIDPrimitive.Top999WiktionaryFr:
        return fetch(
          new URL(
            "/dictionary/top999-wiktionary-fr/words.json",
            import.meta.url
          )
        )
          .then((response) => response.json())
          .then((words) => Object.values(words));
    }
    throw `${this} is not a valid \`UnseenSetID\``;
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
