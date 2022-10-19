import { defineStore } from "pinia";
import UnseenSetID from "@/service/unseen-set-id";
import { Game } from "../../dist/wasm";

const loadGame = (unseenSetID: UnseenSetID) =>
  unseenSetID.words.then(
    (words) => new Game(BigInt(Date.now()), 0.4, unseenSetID.primitive, words)
  );

interface State {
  game: null | Game;
  lives: number;
  score: number;
  currentWord: string;
  _unseenSetID: UnseenSetID;
}

export const useGameStore = defineStore("game", {
  state: () =>
    ({
      game: null,
      lives: Game.initialLivesAmount(),
      score: 0,
      currentWord: "",
      _unseenSetID: UnseenSetID.Top999WiktionaryFr,
    } as State),
  actions: {
    async commitSeen() {
      if (this.game) {
        this.game!.commitSeen();
        this.score = this.game!.score();
        this.lives = this.game!.lives();
      }
    },
    async commitUnseen() {
      if (this.game) {
        this.game!.commitUnseen();
        this.score = this.game!.score();
        this.lives = this.game!.lives();
      }
    },
    async updateCurrentWord() {
      const word = this.game?.next();
      if (word) {
        this.currentWord = word!;
      }
    },
    async setUnseenSet(unseenSetID: UnseenSetID) {
      this.$reset();
      loadGame(unseenSetID).then((game) => {
        this._unseenSetID = unseenSetID;
        this.game = game;
        this.updateCurrentWord();
      });
    },
  },
});
