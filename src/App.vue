<template>
  <div class="m-4">
    <div class="flex justify-evenly">
      <div class="flex">
        <span>score: </span>
        <span>{{ score }}</span>
      </div>
      <div class="flex">
        <span>des vies: </span>
        <span>{{ lives }}</span>
      </div>
    </div>
    <div v-if="lives == 0">
      <div class="grid justify-items-center">
        <p class="bg-red-500 text-white text-8xl p-8">fin</p>
        <gradient-button class="m-6 px-5" @click="reset"
          ><reset-arrow class="w-5 mr-2 mb-1 inline" />encore une
          fois</gradient-button
        >
        <history-table class="m-6" />
      </div>
    </div>
    <div v-else>
      <div class="flex justify-center m-8">
        <p class="text-7xl">{{ currentWord }}</p>
      </div>
      <div class="flex justify-center">
        <div class="grid gap-4 grid-cols-2">
          <gradient-button @click="commitSeen">déjà vu</gradient-button>
          <gradient-button @click="commitNew">nouveau</gradient-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HistoryTable from "./components/HistoryTable.vue";
import GradientButton from "./components/GradientButton.vue";
import ResetArrow from "./assets/ResetArrow.vue";
import { useIndexStore } from "./store";

export default {
  components: { HistoryTable, GradientButton, ResetArrow },
  methods: {
    async commitSeen() {
      await useIndexStore().commitSeen();
      if (this.lives > 0) {
        useIndexStore().updateCurrentWord();
      }
    },
    async commitNew() {
      await useIndexStore().commitUnseen();
      if (this.lives > 0) {
        useIndexStore().updateCurrentWord();
      }
    },
    reset() {
      useIndexStore().$reset();
      useIndexStore().updateCurrentWord();
    },
  },
  computed: {
    currentWord: () => {
      return useIndexStore().currentWord;
    },
    lives: () => {
      return useIndexStore().lives;
    },
    score: () => {
      return useIndexStore().score;
    },
  },
  created() {
    useIndexStore().updateCurrentWord();
  },
};
</script>
