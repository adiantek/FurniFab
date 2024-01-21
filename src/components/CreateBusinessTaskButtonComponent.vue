<script setup lang="ts">
import { computed, ref } from 'vue'
import {
  type BusinessTask,
  type CuttingInfo,
  type FlowInfo,
  getTask,
  nextTaskId,
  useBusinessTasks
} from '@/composables/TaskComposable'
import type { PartialDeep } from '@/utils'

const businessTasks = useBusinessTasks()

const show = ref<boolean>(false)
const newTask = ref<PartialDeep<BusinessTask>>({ cuttingInfo: { conflicts: [] }, flowInfo: {} })

const valid = computed(
  () =>
    newTask.value.name &&
    newTask.value.cuttingInfo &&
    newTask.value.flowInfo &&
    validCuttingInfo(newTask.value.cuttingInfo) &&
    validFlowInfo(newTask.value.flowInfo)
)

function validCuttingInfo(cuttingInfo: PartialDeep<CuttingInfo>): boolean {
  return (
    cuttingInfo.processTime !== undefined &&
    cuttingInfo.weight !== undefined &&
    cuttingInfo.conflicts !== undefined
  )
}

function validFlowInfo(flowInfo: PartialDeep<FlowInfo>): boolean {
  return flowInfo.grindingProcessTime !== undefined && flowInfo.lacqueringProcessTime !== undefined
}

function save() {
  newTask.value.id = nextTaskId.value++

  businessTasks.value.push(newTask.value as BusinessTask)

  for (const conflictId of newTask.value.cuttingInfo!.conflicts!) {
    const conflictTask = getTask(conflictId!)!
    conflictTask.cuttingInfo.conflicts.push(newTask.value.id)
  }

  newTask.value = { cuttingInfo: { conflicts: [] }, flowInfo: {} }
}
</script>

<template>
  <div>
    <button class="btn btn-primary" @click="show = !show">Stwórz zadanie</button>
    <BModal
      v-model="show"
      :okDisabled="!valid"
      okOnly
      okTitle="Zapisz"
      title="Tworzenie zadania"
      @ok="save"
    >
      <BusinessTaskEditorComponent v-model="newTask" />
    </BModal>
  </div>
</template>
