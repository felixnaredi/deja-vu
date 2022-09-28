<!-- eslint-disable -->
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
    <div v-if="lives == 0" class="flex justify-center">
      <p class="bg-red-500 text-white text-8xl p-8">fin</p>
    </div>
    <div v-else>
      <div class="flex justify-center m-8">
        <p class="text-7xl">{{ currentWord }}</p>
      </div>
      <div class="flex justify-center">
        <div class="grid gap-4 grid-cols-2">
          <button
            class="
              p-2
              text-lg
              border-2
              text-white
              bg-gradient-to-t
              from-blue-500
              to-cyan-500
              hover:from-pink-500 hover:to-yellow-500
            "
            @click="commitSeen"
          >
            déjà vu
          </button>
          <button
            class="
              p-2
              text-lg
              border-2
              text-white
              bg-gradient-to-t
              from-blue-500
              to-cyan-500
              hover:from-pink-500 hover:to-yellow-500
            "
            @click="commitNew"
          >
            nouveau
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { useIndexStore } from "./store";

export default {
  data: () => ({}),
  methods: {
    commitSeen() {
      useIndexStore().commitSeen();
      useIndexStore().updateCurrentWord();
    },
    commitNew() {
      useIndexStore().commitNew();
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
