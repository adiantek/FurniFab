<script setup lang="ts">
import { GChart } from 'vue-google-charts'
import { computed } from 'vue'

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
    data.push([`Maszyna ${task.machine}`, task.name, task.tooltip ?? null, task.start, task.end])
  }

  return data
})

const options = computed(() => {
  return {
    hAxis: {
      format: 'HH:mm'
    },
    ...props.chartOptions
  }
})
</script>

<template>
  <div class="w-75 p-5">
    <GChart :settings="settings" type="Timeline" :data="data" :options="options" />
  </div>
</template>

<style scoped></style>
