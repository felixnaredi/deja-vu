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
    <div class="flex justify-center">
      <unseen-set-dropdown class="m-3" :disabled="true" :select="select" />
    </div>
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
import { useGameOverStore } from "@/store/game-over";
import { EncodedGameOver } from "deja-vu-wasm";
import path from "@/service/path";
import ErrorSign from "@/components/ErrorSign.vue";
import UnseenSetDropdown from "@/components/UnseenSetDropdown.vue";
import UnseenSetID from "@/service/unseen-set-id";

export default {
  components: {
    ScoreBoard,
    ResetArrow,
    GameOverTable,
    GradientButton,
    ErrorSign,
    UnseenSetDropdown,
  },
  data: () => ({
    errorMessage: "",
    select: UnseenSetID.Top999WiktionaryFr.primitive,
  }),
  methods: {
    newGame() {
      window.location.href = path(import.meta.env.BASE_URL);
    },
  },
  computed: {
    score: () => useGameOverStore().score,
    lives: () => useGameOverStore().lives,
  },
  created() {
    const encoded = EncodedGameOver.fromURL(window.location.href);

    try {
      new UnseenSetID(encoded.unseenSetID()).words.then((words) => {
        try {
          this.select = encoded.unseenSetID();
          useGameOverStore().setGameOver(
            EncodedGameOver.decode(window.location.href, words)
          );
        } catch (e) {
          console.log(e);
          this.errorMessage = e;
        }
      });
    } catch (e) {
      this.errorMessage = e;
    }
  },
};
</script>
