<!-- eslint-disable -->
<template>
  <div v-if="errorMessage.length > 0">
    <div class="fixed top-0 w-screen z-20 grid grid-cols-12 mt-20">
      <div class="col-span-3"></div>
      <div class="col-span-6 w-full">
        <error-sign class="mt-15"
          ><p>{{ errorMessage }}</p></error-sign
        >
      </div>
      <div class="col-span-3"></div>
    </div>
    <div>
      <div
        class="fixed top-0 z-10 h-screen w-screen bg-slate-300 opacity-75"
      ></div>
    </div>
  </div>
  <div class="z-0">
    <score-board :score="score" :lives="lives" />
    <div class="grid justify-items-center">
      <div class="p-8 bg-red-500 m-4">
        <p class="text-8xl text-white select-none">fin</p>
      </div>
      <gradient-button class="m-4" @click="newGame">
        <div class="flex px-2">
          <reset-arrow class="w-6 mr-4" />
          <span>encore</span>
        </div>
      </gradient-button>
      <game-over-table class="m-4" />
    </div>
  </div>
</template>

<script>
import ResetArrow from "@/assets/ResetArrow.vue";
import GradientButton from "@/components/GradientButton.vue";
import GameOverTable from "@/components/GameOverTable.vue";
import ScoreBoard from "@/components/ScoreBoard.vue";
import { useGameOverStore } from "@/store/history";
import { EncodedGameOver } from "../../../dist/wasm";
import path from "@/service/path";
import ErrorSign from "@/components/ErrorSign.vue";

export default {
  components: {
    ScoreBoard,
    ResetArrow,
    GameOverTable,
    GradientButton,
    ErrorSign,
  },
  data: () => ({
    errorMessage: "",
  }),
  methods: {
    newGame() {
      window.location.href = path(process.env.BASE_URL);
    },
  },
  computed: {
    score: () => useGameOverStore().score,
    lives: () => useGameOverStore().lives,
  },
  created() {
    fetch(path(process.env.BASE_URL, "dictionary", "fr01", "words.json")).then(
      (words) => {
        words.json().then((words) => {
          try {
            const history = EncodedGameOver.decode(window.location.href, words);
            useGameOverStore().setGameOver(history);
          } catch (error) {
            /* eslint-disable-next-line no-console */
            console.error(`Error: EncodedGameOver
          .decode -  ${error}`);
            this.errorMessage = error;
          }
        });
      }
    );
  },
};
</script>
