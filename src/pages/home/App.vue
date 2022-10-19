<template>
  <div class="flex justify-center">
    <unseen-set-dropdown
      class="m-3"
      :disabled="hasPlayedGame"
      :select="select"
      @change="unseenSetChanged"
    />
  </div>
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
import { EncodedGameOver, Game } from "../../../dist/wasm";
import path from "@/service/path";
import UnseenSetDropdown from "@/components/UnseenSetDropdown.vue";
import UnseenSetID from "@/service/unseen-set-id";

export default {
  data: () => ({
    select: UnseenSetID.Top999WiktionaryFr.primitive,
  }),
  components: { GradientButton, ScoreBoard, UnseenSetDropdown },
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
      const encodedGameOver = new EncodedGameOver(
        useGameStore().game.intoGameOver()
      );
      window.location.href = `${path(
        process.env.BASE_URL,
        "game-over"
      )}?${encodedGameOver.asURLSearchParams()}`;
    },
    async unseenSetChanged(event) {
      await useGameStore().setUnseenSet(new UnseenSetID(event.target.value));
      this.select = event.target.value;
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
    hasPlayedGame: () => {
      return !(
        useGameStore().lives == Game.initialLivesAmount() &&
        useGameStore().score == 0
      );
    },
  },
  created() {
    useGameStore().setUnseenSet(UnseenSetID.Top999WiktionaryFr);
  },
};
</script>
