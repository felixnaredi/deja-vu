import { choice, popRandom } from "./choice";
import { PRNG } from "../../dist/wasm";

const DISTRIBUTION_RESOLUTION = 1_000_000;

export default class WordSampler<T> {
  private unseen: Array<T>;
  private seen: Set<T>;
  private seenDistribution: number;
  private _current: null | T;
  private _seed: bigint;
  private rng: PRNG;

  public constructor(unseen: Array<T>, seenDistribution: number) {
    this.unseen = unseen;
    this.seen = new Set();
    this.seenDistribution = seenDistribution * DISTRIBUTION_RESOLUTION;
    this._current = null;
    this._seed = BigInt(Date.now());
    this.rng = PRNG.fromSeed(this._seed);
  }

  /**
   * The last element generated by `next`.
   */
  public get current(): null | T {
    return this._current;
  }

  /**
   * Gets a sample, either seen or unseen. The next time `next` is called, the
   * returned element will be added to the seen ones.
   *
   * @returns A random element.
   */
  public next(): T {
    if (this._current) {
      this.seen.add(this._current!);
    }
    if (
      this.seen.size < 2 ||
      this.rng.nextWithUpperBound(DISTRIBUTION_RESOLUTION) >
        this.seenDistribution
    ) {
      this._current = this.nextUnseen();
    } else {
      this._current = this.nextSeen();
    }
    return this._current!;
  }

  private nextUnseen(): T {
    const x = popRandom(this.rng, this.unseen);
    if (x) {
      return x!;
    } else {
      return this.nextSeen();
    }
  }

  private nextSeen(): T {
    const values = Array.from(this.seen.values());
    let x = choice(this.rng, values)!;
    while (x == this._current!) {
      x = choice(this.rng, values)!;
    }
    return x;
  }

  /**
   * Checks if `element` has been seen.
   *
   * @param element Element to search for.
   * @returns True if `element` has been seen.
   */
  public hasSeen(element: T): boolean {
    return this.seen.has(element);
  }
}
