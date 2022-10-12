import { defineStore } from "pinia";
import { commits } from "@/service/history";
import { History, Commit } from "../../dist/wasm";

interface State {
  history: undefined | History;
}

export const useHistoryStore = defineStore("history", {
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
    setHistory(history: History) {
      this.history = history;
    },
  },
});
