import { defineStore } from "pinia";
import path from "@/service/path";
import { Game } from "../../dist/wasm";

export const useGameStore = defineStore("game", {
  state: () => ({
    game: fetch(
      path(process.env.BASE_URL, "dictionary", "fr01", "words.json")
    ).then((words) =>
      words.json().then((words) => new Game(BigInt(Date.now()), 0.4, words))
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
