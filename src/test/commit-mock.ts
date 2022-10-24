import { Konadare192PxPlusPlus } from "deja-vu-wasm";
import alfanumeric from "@/test/alfanumeric";

/**
 * Pseudo boolean that is either `Seen` or `Unseen`.
 */
interface SeenUnseen {
  /**
   * True if the state is `Seen`.
   * @returns {boolean}
   */
  isSeen(): boolean;
  /**
   * True if the state is `Unseen`.
   * @returns {boolean}
   */
  isUnseen(): boolean;
}

export class Seen {
  public isSeen = () => true;
  public isUnseen = () => false;
}

export class Unseen {
  public isSeen = () => false;
  public isUnseen = () => true;
}

/**
 * Type representing a commit made in a game.
 */
class Commit {
  constructor(element: string, actual: SeenUnseen, guess: SeenUnseen) {
    this.element = () => element;
    this.actual = () => actual;
    this.guess = () => guess;
  }

  /**
   * The value of the element.
   * @returns {string}
   */
  element: () => string;

  /**
   * The actual state of the commit.
   * @returns {SeenUnseen}
   */
  actual: () => SeenUnseen;

  /**
   * The guessed state of the commit.
   * @returns {SeenUnseen}
   */
  guess: () => SeenUnseen;

  /**
   * True if the `guess` is equal to `actual`.
   * @returns {boolean}
   */
  correct(): boolean {
    return this.actual().isSeen() == this.guess().isSeen();
  }
}

interface CommitOptions {
  readonly element?: string;
  readonly actual?: SeenUnseen;
  readonly guess?: SeenUnseen;
  readonly seed?: bigint;
}

export function generateCommit(options: CommitOptions): Commit {
  if (
    options.element != undefined &&
    options.actual != undefined &&
    options.guess != undefined
  ) {
    return new Commit(options.element!, options.actual!, options.guess!);
  } else if (options.seed == undefined) {
    throw "`seed` not set but all fields are not defined";
  } else {
    const rng = new Konadare192PxPlusPlus(options.seed);

    let element = options.element;
    let actual = options.actual;
    let guess = options.guess;

    if (element == undefined) {
      const n = 8 + rng.nextWithUpperBound(16);
      element = "";
      for (let i = 0; i < n; i++) {
        element = `${element}${
          alfanumeric[rng.nextWithUpperBound(alfanumeric.length)]
        }`;
      }
    }

    if (actual == undefined) {
      actual =
        (rng.next() & BigInt(1)) == BigInt(0) ? new Seen() : new Unseen();
    }

    if (guess == undefined) {
      guess = (rng.next() & BigInt(1)) == BigInt(0) ? new Seen() : new Unseen();
    }

    return new Commit(element, actual, guess);
  }
}
