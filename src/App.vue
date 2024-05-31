<script setup lang="ts">
import { exportData, importData, load } from '@/composables/TaskComposable'

document.documentElement.setAttribute('data-bs-theme', 'dark')

load()
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column">
    <BNavbar toggleable="lg" v-b-color-mode="'dark'" class="bg-body-tertiary">
      <BNavbarBrand>FurniFab</BNavbarBrand>
      <BNavbarToggle target="nav-collapse" />
      <BCollapse id="nav-collapse" is-nav>
        <BNavbarNav>
          <router-link to="/" class="nav-link" active-class="active">Zadania</router-link>
          <router-link to="/deliveries" class="nav-link" active-class="active">Dostawy</router-link>
          <router-link to="/binpacking" class="nav-link" active-class="active">Planowanie wycinki</router-link>
          <router-link to="/conflict" class="nav-link" active-class="active">Wycinanie</router-link>
          <router-link to="/flow" class="nav-link" active-class="active">Wykańczanie</router-link>
        </BNavbarNav>
        <!-- Right aligned nav items -->
        <BNavbarNav class="ms-auto mb-2 mb-lg-0">
          <CreateBusinessTaskButtonComponent class="mx-1" />
          <LoadingButton :on-click="importData">Import</LoadingButton>
          <LoadingButton :on-click="exportData">Eksport</LoadingButton>
        </BNavbarNav>
      </BCollapse>
    </BNavbar>
    <div class="w-100 d-flex flex-grow-1 overflow-y-auto flex-column align-items-center p-4 m-auto" style="max-width: 1400px">
      <router-view />
    </div>
  </div>
  <BToaster />
</template>
