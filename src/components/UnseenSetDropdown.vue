<template>
  <div>
    <select
      class="p-2 bg-white border-2 disabled:bg-slate-100"
      :disabled="disabled"
      @change="(event) => $emit('change', event)"
    >
      <option
        v-for="unseenSetID in unseenSetIDs"
        :key="unseenSetID.menuItemLabel"
        :value="unseenSetID.primitive"
      >
        {{ unseenSetID.menuItemLabel }}
      </option>
    </select>
  </div>
</template>

<script>
import UnseenSetID from "@/service/unseen-set-id";

export default {
  computed: {
    unseenSetIDs: () => {
      return [UnseenSetID.Top999WiktionaryFr, UnseenSetID.DictionaryFr01];
    },
  },
  props: {
    disabled: Boolean,
  },
  emits: ["change"],
};
</script>
