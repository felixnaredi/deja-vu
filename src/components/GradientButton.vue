<!-- eslint-disable -->
<template>
  <button
    class="
      p-2
      text-lg
      border-2
      text-white
      bg-gradient-to-t
      from-blue-500
      to-cyan-500
      hover:from-pink-500 hover:to-yellow-500
    "
    @click="(event) => $emit('click', event)"
  >
    <slot></slot>
  </button>
</template>

<script>
export default {
  emits: ["click"],
};
</script>
