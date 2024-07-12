<template>
  <div class="container">
    <Greet />
    <icon-font type="icon-tianqi" :size="64" />
    <icon-font type="icon-zhuye" :size="64"/>
    <icon-font type="icon-fensedemeiguihua" :size="64" :spin=true />
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
