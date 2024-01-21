<script setup lang="ts">
import { type BusinessTask, getTask, useBusinessTasks } from '@/composables/TaskComposable'
import type { PartialDeep } from '@/utils'
import { computed, type Ref, watch } from 'vue'

const model = defineModel<PartialDeep<BusinessTask>>()

const businessTasks: Ref<BusinessTask[]> = useBusinessTasks()

const options = computed(() =>
  businessTasks.value
    .filter((task) => task.id !== model.value?.id)
    .map((task) => ({ value: task.id, text: task.name }))
)

let taskId: number = -1
watch(
  () => model.value?.cuttingInfo?.conflicts,
  (newValue, oldValue) => {
    if (model.value?.id !== undefined && model.value?.id === taskId) {
      for (const conflictId of oldValue!) {
        const conflictTask = getTask(conflictId!)!
        conflictTask.cuttingInfo.conflicts = conflictTask.cuttingInfo.conflicts.filter(
          (conflictId) => conflictId !== model.value?.id
        )
      }

      for (const conflictId of newValue!) {
        const conflictTask = getTask(conflictId!)!
        conflictTask.cuttingInfo.conflicts.push(model.value?.id)
      }
    } else if (model.value?.id !== undefined) {
      taskId = model.value?.id
    }
  }
)
</script>

<template>
  <div v-if="model" class="p-1">
    <div class="input-group mb-2">
      <label class="input-group-text">Nazwa</label>
      <input v-model="model.name" type="text" class="form-control" minlength="1" />
    </div>

    <div v-if="model.cuttingInfo" class="form-group card p-3 mb-2">
      <span class="card-title">Informacje o wycinaniu: </span>

      <div class="input-group mb-2">
        <label class="input-group-text">Czas trwania</label>
        <input v-model="model.cuttingInfo.processTime" type="number" class="form-control" min="1" />
      </div>

      <div class="input-group mb-2">
        <label class="input-group-text">Priorytet</label>
        <input v-model="model.cuttingInfo.weight" type="number" class="form-control" min="0" />
      </div>

      <div class="input-group">
        <label class="input-group-text">Konflikty</label>
        <BFormSelect v-model="model.cuttingInfo.conflicts" :options="options" multiple />
      </div>
    </div>

    <div v-if="model.flowInfo" class="form-group card p-3">
      <span class="card-title">Informacje o lakierowaniu i szlifowaniu: </span>

      <div class="input-group mb-2">
        <label class="input-group-text">Czas szlifowania</label>
        <input
          v-model="model.flowInfo.grindingProcessTime"
          type="number"
          class="form-control"
          min="1"
        />
      </div>

      <div class="input-group">
        <label class="input-group-text">Czas lakierowania</label>
        <input
          v-model="model.flowInfo.lacqueringProcessTime"
          type="number"
          class="form-control"
          min="1"
        />
      </div>
    </div>
  </div>
</template>
