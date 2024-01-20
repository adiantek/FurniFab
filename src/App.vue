<script setup lang="ts">
document.documentElement.setAttribute('data-bs-theme', 'dark')
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column">
    <nav class="nav nav-tabs">
      <div class="nav-item">
        <router-link to="/" class="nav-link" active-class="active">Home</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/tasks" class="nav-link" active-class="active">Zadania</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/conflict" class="nav-link" active-class="active">Wycinanie</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/demo" class="nav-link" active-class="active">Demo</router-link>
      </div>
    </nav>
    <router-view />
  </div>
  <BToaster />
</template>
