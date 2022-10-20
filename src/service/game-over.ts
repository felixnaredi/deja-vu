import { GameOver, Commit } from "deja-vu-wasm";

function GameOverIterator(gameOver: GameOver): Iterable<Commit> {
  const iterator = gameOver.iterator();
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

export function commits(gameOver: GameOver): Commit[] {
  return Array.from(GameOverIterator(gameOver));
}
