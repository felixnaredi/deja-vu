<template>
  <div class="grid grid-cols-10 gap-4 border-b-2">
    <div>{{ actuallyUnseen ? "+" : "" }}</div>
    <div class="col-span-7">{{ entry.value }}</div>
    <div>
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

export default {
  components: { TickSymbol, CrossSymbol },
  props: {
    entry: History.Entry,
  },
  computed: {
    actuallyUnseen() {
      return this.entry.actualState == History.EntryState.Unseen;
    },
    answeredUnseen() {
      return this.entry.answeredState == History.EntryState.Unseen;
    },
    correct() {
      return this.entry.actualState == this.entry.answeredState;
    },
  },
};
</script>
