import { defineStore } from "pinia";
import { choice, popRandom } from "@/service/choice";
import { History, HistoryTracker } from "@/service/history";

export const useIndexStore = defineStore("index", {
  state: () => ({
    lives: 3,
    score: 0,
    seenDistribution: 0.4,
    currentWord: "",
    unseenWords: fetch("/fr/words.json").then((words) => words.json()),
    seenWords: new Array<string>(),
    historyTracker: new HistoryTracker(),
  }),
  actions: {
    commitSeen() {
      this.historyTracker.answer = History.EntryState.Seen;
      if (this.seenWords.includes(this.currentWord)) {
        this.score += 1;
      } else {
        this.lives -= 1;
        this.seenWords.push(this.currentWord);
      }
      return this.lives > 0;
    },
    commitNew() {
      this.historyTracker.answer = History.EntryState.Unseen;
      if (this.seenWords.includes(this.currentWord)) {
        this.lives -= 1;
      } else {
        this.score += 1;
        this.seenWords.push(this.currentWord);
      }
      return this.lives > 0;
    },
    async updateCurrentWord() {
      if (this.seenWords.length < 3) {
        await this.updateToNewWord();
      } else {
        if (Math.random() > this.seenDistribution) {
          await this.updateToNewWord();
        } else {
          await this.updateToSeenWord();
        }
      }
      this.historyTracker.current = this.currentWord;
      console.log(this.historyTracker);
    },
    async updateToNewWord() {
      await this.unseenWords.then(
        (words) => (this.currentWord = popRandom(words)!)
      );
    },
    async updateToSeenWord() {
      const word = choice(this.seenWords)!;
      if (this.currentWord != word) {
        this.currentWord = word;
      } else {
        this.updateToSeenWord();
      }
    },
  },
});
