import { choice, popRandom } from "./choice";

export default class WordSampler<T> {
  private unseen: Array<T>;
  private seen: Set<T>;
  private seenDistribution: number;
  private _current: null | T;

  public constructor(unseen: Array<T>, seenDistribution: number) {
    this.unseen = unseen;
    this.seen = new Set();
    this.seenDistribution = seenDistribution;
    this._current = null;
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
    if (this.seen.size < 2 || Math.random() > this.seenDistribution) {
      this._current = this.nextUnseen();
    } else {
      this._current = this.nextSeen();
    }
    return this._current!;
  }

  private nextUnseen(): T {
    const x = popRandom(this.unseen);
    if (x) {
      return x!;
    } else {
      return this.nextSeen();
    }
  }

  private nextSeen(): T {
    let values = Array.from(this.seen.values());
    let x = choice(values)!;
    while (x == this._current!) {
      x = choice(values)!;
    }
    return x;
  }
}
