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
    <b-form-floating-label label="Nazwa" class="mb-3">
      <b-form-input v-model="model.name" trim placeholder="" />
    </b-form-floating-label>

    <b-card v-if="model.rectInfo" header="Informacje o kształcie elementu" class="mb-3">

      <b-form-floating-label label="Szerokość" class="mb-3">
        <b-form-input type="number" v-model.number="model.rectInfo.w" placeholder="" />
      </b-form-floating-label>

      <b-form-floating-label label="Wysokość" class="mb-3">
        <b-form-input type="number" v-model.number="model.rectInfo.h" placeholder="" />
      </b-form-floating-label>
    
    </b-card>

    <b-card v-if="model.cuttingInfo" header="Informacje o wycinaniu" class="mb-3">

      <b-form-floating-label label="Czas trwania" class="mb-3">
        <b-form-input type="number" v-model.number="model.cuttingInfo.processTime" placeholder="" />
      </b-form-floating-label>

      <b-form-floating-label label="Priorytet" class="mb-3">
        <b-form-input type="number" v-model.number="model.cuttingInfo.weight" placeholder="" />
      </b-form-floating-label>

      <b-input-group>
        <label class="input-group-text">Konflikty:</label>
        <b-form-select v-model="model.cuttingInfo.conflicts" :options="options" multiple />
      </b-input-group>

    </b-card>
    
    <b-card v-if="model.flowInfo" header="Informacje o lakierowaniu i szlifowaniu" class="mb-3">

      <b-form-floating-label label="Czas szlifowania" class="mb-3">
        <b-form-input type="number" v-model.number="model.flowInfo.grindingProcessTime" placeholder="" />
      </b-form-floating-label>

      <b-form-floating-label label="Czas lakierowania" class="mb-3">
        <b-form-input type="number" v-model.number="model.flowInfo.lacqueringProcessTime" placeholder="" />
      </b-form-floating-label>

    </b-card>

    <b-card v-if="model.materialInfo" header="Informacje o materiale" class="mb-3">

      <b-form-floating-label label="Materiał" class="mb-3">
        <b-form-input type="text" v-model.number="model.materialInfo.material" placeholder="" />
      </b-form-floating-label>

      <b-form-floating-label label="Ilość" class="mb-3">
        <b-form-input type="number" v-model.number="model.materialInfo.amount" placeholder="" />
      </b-form-floating-label>

    </b-card>

  </div>
</template>
