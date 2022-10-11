import { commits } from "@/service/history";
import { defineStore } from "pinia";
import { History, Commit } from "../../dist/wasm";

interface State {
  history: null | History;
  score: number;
  lives: number;
  _commits: null | Commit[];
}

export const useHistoryStore = defineStore("history", {
  state: () =>
    ({
      history: null,
      score: 0,
      lives: 0,
      _commits: null,
    } as State),
  getters: {
    commits(): Commit[] {
      if (this._commits == null) {
        if (this.history == null) {
          this._commits = [];
        } else {
          this._commits = commits(this.history);
        }
      }
      return this._commits;
    },
  },
  actions: {
    setHistory(history: History) {
      this.history = history;
      this.score = history.score();
      this.lives = history.lives();
      this._commits = null;
    },
  },
});
