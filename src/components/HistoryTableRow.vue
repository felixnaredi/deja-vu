<template>
  <div class="grid grid-cols-10 gap-4 border-b-2">
    <div>{{ actuallyUnseen ? "+" : "" }}</div>
    <div class="col-span-7">{{ entry.value }}</div>
    <div>{{ !answeredUnseen ? (correct ? "✅" : "❌") : "" }}</div>
    <div>{{ answeredUnseen ? (correct ? "✅" : "❌") : "" }}</div>
  </div>
</template>

<script>
import { History } from "@/service/history";

export default {
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
