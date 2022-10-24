<template>
  <div
    class="grid grid-cols-8 gap-4 border-b-2"
    :class="{ 'bg-red-50': !commit.correct() }"
  >
    <div v-if="commit.actual().isSeen()" class="h-full justify-self-center">
      <eye-symbol class="h-full w-5" />
    </div>
    <div v-else></div>
    <div class="col-span-5 ml-4">{{ commit.element() }}</div>
    <div class="justify-self-center">
      <div v-if="commit.guess().isSeen()" class="h-full">
        <tick-symbol v-if="commit.correct()" class="w-5 h-full" />
        <cross-symbol v-else class="w-5 h-full" />
      </div>
      <div v-else></div>
    </div>
    <div v-if="commit.guess().isUnseen()" class="h-full justify-self-center">
      <tick-symbol v-if="commit.correct()" class="w-5 h-full" />
      <cross-symbol v-else class="w-5 h-full" />
    </div>
    <div v-else></div>
  </div>
</template>

<script>
import TickSymbol from "@/assets/TickSymbol.vue";
import CrossSymbol from "@/assets/CrossSymbol.vue";
import EyeSymbol from "@/assets/EyeSymbol.vue";
import Commit from "@/interfaces/_Commit";

export default {
  components: { TickSymbol, CrossSymbol, EyeSymbol },
  props: {
    commit: Commit,
  },
};
</script>
