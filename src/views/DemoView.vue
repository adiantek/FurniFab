<script setup lang="ts">
import { ref } from 'vue'
import { type CommandOutput, runExecutable, runFlow } from '@/api'

const commandResult = ref<CommandOutput | undefined>()
const result = ref<string | undefined>()

async function onButtonClick(exec: string) {
  commandResult.value = await runExecutable(exec, 'file_pa.txt\n').catch((error) => {
    console.error(error)
    return undefined
  })
}

async function onButtonClick2() {
  result.value = await runFlow()
}
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center">
    <button @click="() => onButtonClick('pa')">Run F2</button>
    <button @click="onButtonClick2">Run pa</button>
    <div>{{ commandResult?.stdout }}</div>
    <div>{{ commandResult?.stderr }}</div>
    <div>{{ commandResult?.error }}</div>
    <div>{{ result }}</div>
  </div>
</template>

<style scoped></style>
