<script setup lang="ts">
import { ref } from 'vue'
import { type CommandOutput, runExecutable } from '@/api'

document.documentElement.setAttribute('data-bs-theme', 'dark')

const commandResult = ref<CommandOutput | undefined>()

async function onButtonClick(exec: string) {
  commandResult.value = await runExecutable(exec, 'file_pa.txt\n').catch((error) => {
    console.error(error)
    return undefined
  })
}
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
    <div>Tu będzie projekt</div>
    <div>
      <button @click="() => onButtonClick('pa.exe')">Run F2</button>
      <br>
      <button @click="() => onButtonClick('scheduling_conflicts.exe')">
        Run scheduling with conflicts
      </button>
    </div>
    <div>{{ commandResult?.stdout }}</div>
    <div>{{ commandResult?.stderr }}</div>
    <div>{{ commandResult?.error }}</div>
  </div>
  <BToaster />
</template>
