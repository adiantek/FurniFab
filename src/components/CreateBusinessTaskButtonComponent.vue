<script setup lang="ts">
import { computed, ref } from 'vue'
import {
  type BusinessTask,
  type CuttingInfo,
  type FlowInfo,
  type RectInfo,
  getTask,
  nextTaskId,
  useBusinessTasks
} from '@/composables/TaskComposable'
import type { PartialDeep } from '@/utils'

const businessTasks = useBusinessTasks()

const show = ref<boolean>(false)
const newTask = ref<PartialDeep<BusinessTask>>({ cuttingInfo: { conflicts: [] }, flowInfo: {}, rectInfo: {} })

const valid = computed(
  () =>
    newTask.value.name &&
    newTask.value.cuttingInfo &&
    newTask.value.flowInfo &&
    newTask.value.rectInfo &&
    validCuttingInfo(newTask.value.cuttingInfo) &&
    validFlowInfo(newTask.value.flowInfo) &&
    validRectInfo(newTask.value.rectInfo)
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

function validRectInfo(rectInfo: PartialDeep<RectInfo>): boolean {
  return rectInfo.w !== undefined && rectInfo.h !== undefined && rectInfo.w > 0 && rectInfo.h > 0
}

function save() {
  newTask.value.id = nextTaskId.value++

  businessTasks.value.push(newTask.value as BusinessTask)

  for (const conflictId of newTask.value.cuttingInfo!.conflicts!) {
    const conflictTask = getTask(conflictId!)!
    conflictTask.cuttingInfo.conflicts.push(newTask.value.id)
  }

  newTask.value = { cuttingInfo: { conflicts: [] }, flowInfo: {},  rectInfo: {} }
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
