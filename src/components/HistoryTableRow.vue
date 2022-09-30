<template>
  <div
    class="grid grid-cols-8 gap-4 border-b-2"
    :class="{ 'bg-red-50': incorrect }"
  >
    <div v-if="actuallySeen" class="h-full justify-self-center">
      <eye-symbol class="h-full w-5" />
    </div>
    <div v-else></div>
    <div class="col-span-5 ml-4">{{ entry.value }}</div>
    <div class="justify-self-end">
      <div v-if="!answeredUnseen" class="h-full">
        <tick-symbol v-if="correct" class="w-5 h-full" />
        <cross-symbol v-else class="w-5 h-full" />
      </div>
      <div v-else></div>
    </div>
    <div v-if="answeredUnseen" class="h-full">
      <tick-symbol v-if="correct" class="w-5 h-full" />
      <cross-symbol v-else class="w-5 h-full" />
    </div>
    <div v-else></div>
  </div>
</template>

<script>
import { History } from "@/service/history";
import TickSymbol from "../assets/TickSymbol.vue";
import CrossSymbol from "../assets/CrossSymbol.vue";
import EyeSymbol from "@/assets/EyeSymbol.vue";

export default {
  components: { TickSymbol, CrossSymbol, EyeSymbol },
  props: {
    entry: History.Entry,
  },
  computed: {
    actuallySeen() {
      return this.entry.actualState == History.EntryState.Seen;
    },
    answeredUnseen() {
      return this.entry.answeredState == History.EntryState.Unseen;
    },
    correct() {
      return this.entry.actualState == this.entry.answeredState;
    },
    incorrect() {
      return !this.correct;
    },
  },
};
</script>
