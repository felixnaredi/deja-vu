import { defineStore } from "pinia";
import { commits } from "@/service/game-over";
import { GameOver, Commit } from "deja-vu-wasm";

interface State {
  gameOver: undefined | GameOver;
}

export const useGameOverStore = defineStore("game-over", {
  state: () =>
    ({
      gameOver: undefined,
    } as State),
  getters: {
    commits(): undefined | Commit[] {
      if (this.gameOver == undefined) {
        return undefined;
      } else {
        return commits(this.gameOver);
      }
    },
    score(): undefined | number {
      return this.gameOver?.score();
    },
    lives(): undefined | number {
      return this.gameOver?.lives();
    },
  },
  actions: {
    setGameOver(gameOver: GameOver) {
      this.gameOver = gameOver;
    },
  },
});
