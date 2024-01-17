<script setup lang="ts"></script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center loader">
    <div class="spinner-border" role="status">
      <span class="visually-hidden"></span>
    </div>
    <div>Tu będzie projekt</div>
  </div>
</template>

<style scoped>
.loader {
  background-color: var(--bs-body-bg);
  z-index: 10;
}
</style>
