<template>
  <div>
    <div class="grid grid-cols-8 gap-4">
      <span>première</span>
      <div class="col-span-5"></div>
      <span class="justify-self-center">déjà vu</span>
      <span class="justify-self-center">nouveau</span>
    </div>
    <history-table-row
      v-for="entry in entries"
      :key="entry.index"
      :entry="entry.value"
    />
  </div>
</template>

<script>
import { useIndexStore } from "@/store";
import HistoryTableRow from "./HistoryTableRow.vue";

export default {
  components: { HistoryTableRow },
  computed: {
    entries: () => {
      return useIndexStore().historyTracker.entries.map((value, index) => {
        return {
          index,
          value,
        };
      });
    },
  },
  created() {
    console.log(this.entries);
  },
};
</script>
