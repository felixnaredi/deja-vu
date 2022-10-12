<template>
  <score-board :score="score" :lives="lives" />
  <div>
    <div class="flex justify-center m-8">
      <p class="text-7xl">{{ currentWord }}</p>
    </div>
    <div class="flex justify-center">
      <div class="grid gap-4 grid-cols-2">
        <gradient-button @click="commitSeen">déjà vu</gradient-button>
        <gradient-button @click="commitUnseen">nouveau</gradient-button>
      </div>
    </div>
  </div>
</template>

<script>
import GradientButton from "@/components/GradientButton.vue";
import ScoreBoard from "@/components/ScoreBoard.vue";
import { useGameStore } from "@/store/game";
import { encode } from "../../../dist/wasm";

export default {
  components: { GradientButton, ScoreBoard },
  methods: {
    async commitSeen() {
      await useGameStore().commitSeen();
      if (this.lives > 0) {
        useGameStore().updateCurrentWord();
      } else {
        this.goToHistory();
      }
    },
    async commitUnseen() {
      await useGameStore().commitUnseen();
      if (this.lives > 0) {
        useGameStore().updateCurrentWord();
      } else {
        this.goToHistory();
      }
    },
    reset() {
      useGameStore().$reset();
      useGameStore().updateCurrentWord();
    },
    async goToHistory() {
      console.log(encode((await useGameStore().game).intoHistory()));
    },
  },
  computed: {
    currentWord: () => {
      return useGameStore().currentWord;
    },
    lives: () => {
      return useGameStore().lives;
    },
    score: () => {
      return useGameStore().score;
    },
  },
  created() {
    useGameStore().updateCurrentWord();
  },
};
</script>
