<!-- eslint-disable -->
<template>
  <div
    class="
      fixed
      top-0
      w-screen
      z-20
      grid grid-cols-12
      mt-20
      opacity-95
      hover:opacity-100
    "
  >
    <div class="col-span-3"></div>
    <div class="col-span-6 place-self-center">
      <error-sign
        class="mt-15"
        message="There isn't really an error. The sign is just for display."
      />
    </div>
    <div class="col-span-3"></div>
  </div>
  <div>
    <div
      class="fixed top-0 z-10 h-screen w-screen bg-slate-300 opacity-75"
    ></div>
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
      <history-table class="m-4" />
    </div>
  </div>
</template>

<script>
import ResetArrow from "@/assets/ResetArrow.vue";
import GradientButton from "@/components/GradientButton.vue";
import HistoryTable from "@/components/HistoryTable.vue";
import ScoreBoard from "@/components/ScoreBoard.vue";
import { useHistoryStore } from "@/store/history";
import { Encoded } from "../../../dist/wasm";
import path from "@/service/path";
import ErrorSign from "@/components/ErrorSign.vue";

export default {
  components: {
    ScoreBoard,
    ResetArrow,
    HistoryTable,
    GradientButton,
    ErrorSign,
  },
  methods: {
    newGame() {
      window.location.href = process.env.BASE_URL;
    },
  },
  computed: {
    score: () => useHistoryStore().score,
    lives: () => useHistoryStore().lives,
  },
  created: () => {
    fetch(path(process.env.BASE_URL, "dictionary", "fr01", "words.json")).then(
      (words) => {
        words.json().then((words) => {
          const history = Encoded.decode(window.location.href, words);
          useHistoryStore().setHistory(history);
        });
      }
    );
  },
};
</script>
