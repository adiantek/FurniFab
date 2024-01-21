<script setup lang="ts">
import { GChart } from 'vue-google-charts'
import { computed, ref } from 'vue'

export interface ScheduledTask {
  machine: number
  name: string
  start: Date
  end: Date
  tooltip?: string
}

const settings: { packages: any[] } = {
  packages: ['timeline']
}

const props = defineProps<{
  tasks: ScheduledTask[]
  chartOptions?: any
}>()

const width = ref<number>(1500)

const data = computed(() => {
  const data: unknown[][] = [
    [
      { type: 'string', label: 'Maszyna', id: 'Machine' },
      { type: 'string', label: 'Nazwa', id: 'Name' },
      { type: 'string', id: 'Tooltip', role: 'tooltip' },
      { type: 'datetime', label: 'Początek', id: 'Start' },
      { type: 'datetime', label: 'Koniec', id: 'End' }
    ]
  ]

  for (const task of props.tasks) {
    data.push([`Pracownik ${task.machine}`, task.name, task.tooltip ?? null, task.start, task.end])
  }

  return data
})

const options = computed(() => {
  return {
    hAxis: {
      format: 'HH:mm'
    },
    width: width.value,
    ...props.chartOptions
  }
})

function zoomIn() {
  width.value *= 1.2
}

function zoomOut() {
  width.value *= 0.8
}

function zoomReset() {
  width.value = 1500
}
</script>

<template>
  <div class="input-group m-2">
    <button class="btn btn-secondary ms-auto me-1" @click="zoomIn">
      <ZoomInIconComponent />
    </button>
    <button class="btn btn-secondary my-auto" @click="zoomOut">
      <ZoomOutIconComponent />
    </button>
    <button class="btn btn-secondary m-auto ms-1" @click="zoomReset">
      <ZoomResetIconComponent />
    </button>
  </div>
  <GChart
    :settings="settings"
    type="Timeline"
    :data="data"
    :options="options"
    class="w-100 p-1 overflow-x-auto overflow-y-hidden"
    style="height: 200px"
  />
</template>

<style scoped></style>
