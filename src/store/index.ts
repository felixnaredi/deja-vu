import { defineStore } from "pinia";
import { History, HistoryTracker } from "@/service/history";
import { Game } from "../../dist/wasm";

export const useIndexStore = defineStore("index", {
  state: () => ({
    game: fetch("deja-vu/dictionary/fr/words.json").then((words) =>
      words.json().then((words) => new Game(BigInt(Date.now()), 0.4, words))
    ),
    lives: Game.initialLivesAmount(),
    score: 0,
    currentWord: "",
    historyTracker: new HistoryTracker(),
  }),
  actions: {
    async commitSeen() {
      this.historyTracker.answer = History.EntryState.Seen;

      this.game.then((game) => {
        game.commitSeen();
        this.score = game.score();
        this.lives = game.lives();
      });
    },
    async commitUnseen() {
      this.historyTracker.answer = History.EntryState.Unseen;

      this.game.then((game) => {
        game.commitUnseen();
        this.score = game.score();
        this.lives = game.lives();
      });
    },
    async updateCurrentWord() {
      const word = (await this.game).next();
      this.historyTracker.current = word;
      this.currentWord = word;
    },
  },
});
