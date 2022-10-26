<!-- eslint-disable -->
<template>
  <div
    class="
      p-8
      bg-gradient-to-t
      from-amber-200
      to-amber-100
      border-solid border-2 border-sky-500
      transition
      ease-in
      opacity-90
      duration-500
      hover:opacity-100
    "
  >
    <h1 class="text-red-500 font-bold text-lg mb-3">Error</h1>
    <div class="mb-16">
      <slot></slot>
    </div>
    <button
      class="
        border-2
        p-4
        py-3
        border-amber-400
        hover:border-amber-500 hover:bg-amber-300/[.25]
        cursor-default
        select-none
      "
      @click="goHome"
    >
      D'accueil
    </button>
  </div>
</template>

<script>
import path from "@/service/path";
export default {
  methods: {
    goHome: () => {
      window.location.href = path(import.meta.env.BASE_URL);
    },
  },
};
</script>
