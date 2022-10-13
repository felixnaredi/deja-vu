import { defineStore } from "pinia";
import { commits } from "@/service/game-over";
import { GameOver, Commit } from "../../dist/wasm";

interface State {
  history: undefined | GameOver;
}

export const useGameOverStore = defineStore("history", {
  state: () =>
    ({
      history: undefined,
    } as State),
  getters: {
    commits(): undefined | Commit[] {
      if (this.history == undefined) {
        return undefined;
      } else {
        return commits(this.history);
      }
    },
    score(): undefined | number {
      return this.history?.score();
    },
    lives(): undefined | number {
      return this.history?.lives();
    },
  },
  actions: {
    setGameOver(history: GameOver) {
      this.history = history;
    },
  },
});
