import { defineStore } from "pinia";
import UnseenSetID from "@/service/unseen-set-id";
import { Game } from "../../dist/wasm";

export const useGameStore = defineStore("game", {
  state: () => ({
    game: UnseenSetID.DictionaryFr01().words.then(
      (words) => new Game(BigInt(Date.now()), 0.4, words)
    ),
    lives: Game.initialLivesAmount(),
    score: 0,
    currentWord: "",
  }),
  actions: {
    async commitSeen() {
      this.game.then((game) => {
        game.commitSeen();
        this.score = game.score();
        this.lives = game.lives();
      });
    },
    async commitUnseen() {
      this.game.then((game) => {
        game.commitUnseen();
        this.score = game.score();
        this.lives = game.lives();
      });
    },
    async updateCurrentWord() {
      const word = (await this.game).next();
      this.currentWord = word;
    },
  },
});
