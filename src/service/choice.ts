/**
 * Chooses a random element from `array` and returns it.
 *
 * @param array The array to pick an element from.
 * @returns A random element.
 */
export function choice<T>(array: Array<T>): null | T {
  if (array.length == 0) {
    return null;
  } else {
    return array[Math.floor(Math.random() * array.length)];
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
export function popRandom<T>(array: Array<T>): null | T {
  if (array.length == 0) {
    return null;
  } else {
    const i = Math.floor(Math.random() * array.length);
    const x = array[i];
    array[i] = array.pop()!;
    return x;
  }
}
