import { defineStore } from "pinia";
import { choice, popRandom } from "@/service/choice";

export const useIndexStore = defineStore("index", {
  state: () => ({
    lives: 3,
    score: 0,
    seenDistribution: 0.4,
    currentWord: "",
    unseenWords: fetch("/fr/words.json").then((words) => words.json()),
    seenWords: new Array<string>(),
  }),
  actions: {
    commitSeen() {
      if (this.seenWords.includes(this.currentWord)) {
        this.score += 1;
      } else {
        this.lives -= 1;
        this.seenWords.push(this.currentWord);
      }
    },
    commitNew() {
      if (this.seenWords.includes(this.currentWord)) {
        this.lives -= 1;
      } else {
        this.score += 1;
        this.seenWords.push(this.currentWord);
      }
    },
    async updateCurrentWord() {
      if (this.seenWords.length < 3) {
        this.updateToNewWord();
      } else {
        if (Math.random() > this.seenDistribution) {
          this.updateToNewWord();
        } else {
          this.updateToSeenWord();
        }
      }
    },
    updateToNewWord() {
      this.unseenWords.then((words) => (this.currentWord = popRandom(words)!));
    },
    updateToSeenWord() {
      const word = choice(this.seenWords)!;
      if (this.currentWord != word) {
        this.currentWord = word;
      } else {
        this.updateCurrentWord();
      }
    },
  },
});
