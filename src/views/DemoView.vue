<script setup lang="ts">
import { ref } from 'vue'
import { type CommandOutput, runExecutable } from '@/api'

const commandResult = ref<CommandOutput | undefined>()
const result = ref<string | undefined>()

async function onButtonClick(exec: string) {
  commandResult.value = await runExecutable(exec, 'file_pa.txt\n').catch((error) => {
    console.error(error)
    return undefined
  })
}
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center">
    <button @click="() => onButtonClick('pa')">Run F2</button>
    <div>{{ commandResult?.stdout }}</div>
    <div>{{ commandResult?.stderr }}</div>
    <div>{{ commandResult?.error }}</div>
    <div>{{ result }}</div>
  </div>
</template>

<style scoped></style>
