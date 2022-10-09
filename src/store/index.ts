import { defineStore } from "pinia";
import { History, HistoryTracker } from "@/service/history";
import { Game } from "../../dist/wasm";

export const useIndexStore = defineStore("index", {
  state: () => ({
    game: fetch("deja-vu/dictionary/fr/words.json").then((words) =>
      words.json().then((words) => new Game(BigInt(Date.now()), 0.4, words))
    ),
    lives: 3,
    score: 0,
    currentWord: "",
    historyTracker: new HistoryTracker(),
  }),
  actions: {
    async commitSeen() {
      this.historyTracker.answer = History.EntryState.Seen;
      if ((await this.game).commitSeen()) {
        this.score += 1;
      } else {
        this.lives -= 1;
      }
    },
    async commitUnseen() {
      this.historyTracker.answer = History.EntryState.Unseen;
      if ((await this.game).commitUnseen()) {
        this.score += 1;
      } else {
        this.lives -= 1;
      }
    },
    async updateCurrentWord() {
      const word = (await this.game).next();
      this.historyTracker.current = word;
      this.currentWord = word;
    },
  },
});
