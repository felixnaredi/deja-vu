import { PRNG } from "../../dist/wasm";

/**
 * Chooses a random element from `array` and returns it.
 *
 * @param array The array to pick an element from.
 * @returns A random element.
 */
export function choice<T>(rng: PRNG, array: Array<T>): null | T {
  if (array.length == 0) {
    return null;
  } else {
    return array[rng.nextWithUpperBound(array.length)];
  }
}

/**
 * Pops a random element from `array`.
 *
 * NOTE:
 *   This function does not keep the order of the elements the array.
 *
 * @param array The array to pop an element from.
 * @returns A random element removed from `array`.
 */
export function popRandom<T>(rng: PRNG, array: Array<T>): null | T {
  if (array.length == 0) {
    return null;
  } else {
    const i = rng.nextWithUpperBound(array.length);
    const x = array[i];
    if (i < array.length - 1) {
      array[i] = array.pop()!;
    } else {
      array.pop();
    }
    return x;
  }
}
