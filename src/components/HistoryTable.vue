<template>
  <div>
    <div class="grid grid-cols-8 gap-4 mb-3">
      <div class="col-span-6"></div>
      <span class="justify-self-center underline">déjà vu</span>
      <span class="justify-self-center underline">nouveau</span>
    </div>
    <history-table-row
      v-for="commit in commits"
      :key="commit.index"
      :commit="commit.value"
    />
  </div>
</template>

<script>
import { useHistoryStore } from "@/store/history";
import HistoryTableRow from "./HistoryTableRow.vue";

export default {
  components: { HistoryTableRow },
  computed: {
    commits: () => {
      return useHistoryStore().commits?.map((value, index) => ({
        index,
        value,
      }));
    },
  },
};
</script>
