<template>
  <div class="grid grid-cols-3 mx-10">
    <div class="justify-self-center w-1/2">
      <span class="mr-1">score:</span>
      <span>{{ score }}</span>
    </div>
    <div></div>
    <div class="justify-self-center w-1/2">
      <span class="mr-1">des vies:</span>
      <span>{{ lives }}</span>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    score: Number,
    lives: Number,
  },
};
</script>
