import { defineStore } from "pinia";

export const useIndexStore = defineStore("index", {
  state: () => ({
    seenWords: new Set<string>(),
    lives: 3,
    score: 0,
    seenDistribution: 0.4,
    currentWord: "",
  }),
  actions: {
    commitSeen() {
      if (this.seenWords.has(this.currentWord)) {
        this.score += 1;
      } else {
        this.lives -= 1;
        this.seenWords.add(this.currentWord);
      }
    },
    commitNew() {
      if (this.seenWords.has(this.currentWord)) {
        this.lives -= 1;
      } else {
        this.score += 1;
        this.seenWords.add(this.currentWord);
      }
    },
    async updateCurrentWord() {
      if (this.seenWords.size < 3) {
        this.updateToNewWord();
      } else {
        console.log(Math.random());
        console.log(this.seenDistribution);
        if (Math.random() > this.seenDistribution) {
          this.updateToNewWord();
        } else {
          console.log("seen");
          this.updateToSeenWord();
        }
      }
    },
    updateToNewWord() {
      fetch("/fr/words.json").then((response: Response) => {
        response.json().then((words) => {
          const newWords = words.filter(
            (word: string) => !this.seenWords.has(word)
          );
          this.currentWord =
            newWords[Math.floor(Math.random() * newWords.length)];
        });
      });
    },
    updateToSeenWord() {
      let i = Math.floor(Math.random() * this.seenWords.size);
      for (const word of this.seenWords.values()) {
        if (i == 0) {
          if (this.currentWord == word) {
            this.updateToSeenWord();
          } else {
            this.currentWord = word;
          }
          return;
        }
        i -= 1;
      }
    },
  },
});
