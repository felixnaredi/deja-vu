<template>
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
</template>

<script>
import GradientButton from "@/components/GradientButton.vue";
import HistoryTable from "@/components/HistoryTable.vue";
import ResetArrow from "@/assets/ResetArrow.vue";
import ScoreBoard from "@/components/ScoreBoard.vue";
import { useGameStore } from "@/store/game";
import { useHistoryStore } from "@/store/history";

export default {
  components: { ScoreBoard, ResetArrow, HistoryTable, GradientButton },
  methods: {
    newGame() {
      useGameStore().$reset();
      this.$router.push("/");
    },
  },
  computed: {
    score: () => useHistoryStore().score,
    lives: () => useHistoryStore().lives,
  },
};
</script>
