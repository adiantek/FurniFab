<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  onClick: () => Promise<void>
  disabled?: boolean
}>()

const loading = ref<boolean>(false)

async function click() {
  loading.value = true
  try {
    await props.onClick()
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <button class="btn btn-primary m-auto me-1" :disabled="loading || disabled" @click="click">
    <span v-if="loading" class="spinner-border spinner-border-sm" role="status"></span>
    <slot v-else></slot>
  </button>
</template>
