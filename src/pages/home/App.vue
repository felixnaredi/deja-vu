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
import { Encoded } from "../../../dist/wasm";
import path from "@/service/path";

export default {
  components: { GradientButton, ScoreBoard },
  methods: {
    async commitSeen() {
      await useGameStore().commitSeen();
      if (this.lives > 0) {
        useGameStore().updateCurrentWord();
      } else {
        this.goToGameOver();
      }
    },
    async commitUnseen() {
      await useGameStore().commitUnseen();
      if (this.lives > 0) {
        useGameStore().updateCurrentWord();
      } else {
        this.goToGameOver();
      }
    },
    reset() {
      useGameStore().$reset();
      useGameStore().updateCurrentWord();
    },
    async goToGameOver() {
      const encoded = new Encoded((await useGameStore().game).intoGameOver());
      window.location.href = `${path(
        process.env.BASE_URL,
        "game-over"
      )}?${encoded.asURLSearchParams()}`;
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
