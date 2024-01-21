<script setup lang="ts">
import { computed } from 'vue'

const model = defineModel<Date>()

const dateString = computed(() => {
  return model.value!.toISOString().slice(0, 10)
})

function onInput(event: Event) {
  const target = event.target as HTMLInputElement
  const newDate = new Date(target.value)
  model.value!.setFullYear(newDate.getFullYear(), newDate.getMonth(), newDate.getDate())
}

function backward() {
  model.value! = new Date(model.value!.setDate(model.value!.getDate() - 1))
}

function forward() {
  model.value! = new Date(model.value!.setDate(model.value!.getDate() + 1))
}
</script>

<template>
  <div class="input-group">
    <button class="btn btn-secondary m-auto" @click="backward">&lt;</button>
    <input :value="dateString" @input="onInput" type="date" class="form-control justify-content-center" />
    <button class="btn btn-secondary m-auto" @click="forward">&gt;</button>
  </div>
</template>
