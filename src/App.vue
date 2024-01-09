<script setup lang="ts">

document.documentElement.setAttribute('data-bs-theme', 'dark');

</script>
<style scoped>
.loader {
  background-color: var(--bs-body-bg);
  z-index: 10;
}
</style>
<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center loader">
    <div class="spinner-border" role="status">
      <span class="visually-hidden"></span>
    </div>
    <div>
      Tu będzie projekt
    </div>
  </div>
  <BToaster />
</template>
