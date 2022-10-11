import { History, Commit } from "../../dist/wasm";

function HistoryIterator(history: History): Iterable<Commit> {
  const iterator = history.intoIterator();
  return {
    [Symbol.iterator]: function* () {
      do {
        const x = iterator.next();
        if (x == null) {
          break;
        } else {
          yield x;
        }
      } while (true);
    },
  };
}

export function commits(history: History): Commit[] {
  return Array.from(HistoryIterator(history));
}
