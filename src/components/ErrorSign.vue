<!-- eslint-disable -->
<template>
  <div class="p-8 bg-amber-200 border-solid border-2 border-sky-500">
    <h1 class="text-red-500 font-bold text-lg mb-3">Error</h1>
    <p class="mb-16">{{ message }}</p>
    <span
      class="
        border-2
        p-4
        py-3
        border-amber-400
        hover:border-amber-500 hover:bg-amber-300/[.25]
        cursor-default
        select-none
      "
      >D'accueil</span
    >
  </div>
</template>

<script>
export default {
  props: {
    message: String,
  },
};
</script>
