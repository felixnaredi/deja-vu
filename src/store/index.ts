import { defineStore } from "pinia";
import { History, HistoryTracker } from "@/service/history";
import WordSampler from "@/service/word-sampler";

export const useIndexStore = defineStore("index", {
  state: () => ({
    lives: 3,
    score: 0,
    currentWord: "",
    sampler: fetch("deja-vu/dictionary/fr/words.json").then((words) =>
      words.json().then((words) => new WordSampler<string>(words, 0.4))
    ),
    historyTracker: new HistoryTracker(),
  }),
  actions: {
    async commitSeen() {
      this.historyTracker.answer = History.EntryState.Seen;
      this.sampler.then((sampler) => {
        if (sampler.hasSeen(sampler.current!)) {
          this.score += 1;
        } else {
          this.lives -= 1;
        }
      });
    },
    async commitUnseen() {
      this.historyTracker.answer = History.EntryState.Unseen;
      this.sampler.then((sampler) => {
        if (sampler.hasSeen(sampler.current!)) {
          this.lives -= 1;
        } else {
          this.score += 1;
        }
      });
    },
    async updateCurrentWord() {
      this.sampler.then((sampler) => {
        this.historyTracker.current = sampler.next();
        this.currentWord = sampler.current!;
      });
    },
  },
});
