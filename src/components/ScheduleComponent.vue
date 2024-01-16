<script setup lang="ts">
import { GChart } from 'vue-google-charts'
import { ref } from 'vue'
import {
  type GoogleChartOptions,
  type GoogleDataTableColumn,
  GoogleDataTableColumnRoleType
} from 'vue-google-charts/dist/types'

export interface ScheduledTask {
  machine: number
  name: string
  start: Date
  end: Date
  tooltip?: string
}

const settings = {
  packages: ['timeline']
}

const props = defineProps<{
  tasks: ScheduledTask[]
  chartOptions?: GoogleChartOptions
}>()

const data = ref<(null | string | Date | GoogleDataTableColumn)[][]>([
  [
    { type: 'string', label: 'Maszyna', id: 'Machine' },
    { type: 'string', label: 'Nazwa', id: 'Name' },
    { type: 'string', id: 'Tooltip', role: 'tooltip' as GoogleDataTableColumnRoleType },
    { type: 'datetime', label: 'Początek', id: 'Start' },
    { type: 'datetime', label: 'Koniec', id: 'End' }
  ]
])

const options = ref<GoogleChartOptions>({
  hAxis: {
    format: 'HH:mm'
  }
})

for (const task of props.tasks) {
  data.value.push([
    `Maszyna ${task.machine}`,
    task.name,
    task.tooltip ?? null,
    task.start,
    task.end
  ])
}

if (props.chartOptions) {
  options.value = {
    ...options.value,
    ...props.chartOptions
  }
}
</script>

<template>
  <div class="chart-container">
    <GChart :settings="settings" type="Timeline" :data="data" :options="options" />
  </div>
</template>

<style scoped>
.chart-container {
  width: 100%;
  padding: 10px;
  box-sizing: border-box;
}
</style>
