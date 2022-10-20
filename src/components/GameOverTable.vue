<template>
  <div>
    <div class="grid grid-cols-8 gap-4 mb-3">
      <div class="col-span-6"></div>
      <span class="justify-self-center underline">déjà vu</span>
      <span class="justify-self-center underline">nouveau</span>
    </div>
    <game-over-table-row
      v-for="commit in commits"
      :key="commit.index"
      :commit="commit.value"
    />
  </div>
</template>

<script>
import { useGameOverStore } from "@/store/game-over";
import GameOverTableRow from "@/components/GameOverTableRow.vue";

export default {
  components: { GameOverTableRow },
  computed: {
    commits: () => {
      return useGameOverStore().commits?.map((value, index) => ({
        index,
        value,
      }));
    },
  },
};
</script>
