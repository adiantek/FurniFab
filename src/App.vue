<script setup lang="ts">
import { exportData, importData, load } from '@/composables/TaskComposable'

document.documentElement.setAttribute('data-bs-theme', 'dark')

load()
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column">
    <nav class="nav nav-tabs">
      <div class="nav-item">
        <router-link to="/" class="nav-link" active-class="active">Zadania</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/deliveries" class="nav-link" active-class="active">Dostawy</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/binpacking" class="nav-link" active-class="active">Planowanie wycinki</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/conflict" class="nav-link" active-class="active">Wycinanie</router-link>
      </div>
      <div class="nav-item">
        <router-link to="/flow" class="nav-link" active-class="active">Wykańczanie</router-link>
      </div>
      <CreateBusinessTaskButtonComponent />
      <div>
        <LoadingButton :on-click="importData">Import</LoadingButton>
      </div>
      <div>
        <LoadingButton :on-click="exportData">Eksport</LoadingButton>
      </div>
    </nav>
    <div class="w-100 d-flex flex-grow-1 flex-column align-items-center p-4 m-auto"
      style="max-width: 1400px">
      <router-view />
    </div>
  </div>
  <BToaster />
</template>
