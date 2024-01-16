<script setup lang="ts">
import { GChart } from 'vue-google-charts'
import { computed } from 'vue'
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

const data = computed<(null | string | Date | GoogleDataTableColumn)[][]>(() => {
  const data = [
    [
      { type: 'string', label: 'Maszyna', id: 'Machine' },
      { type: 'string', label: 'Nazwa', id: 'Name' },
      { type: 'string', id: 'Tooltip', role: 'tooltip' as GoogleDataTableColumnRoleType },
      { type: 'datetime', label: 'Początek', id: 'Start' },
      { type: 'datetime', label: 'Koniec', id: 'End' }
    ]
  ]

  for (const task of props.tasks) {
    data.push([
      `Maszyna ${task.machine}`,
      task.name,
      task.tooltip ?? null,
      task.start,
      task.end
    ])
  }

  return data
})

const options = computed<GoogleChartOptions>(() => {
  return {
    hAxis: {
      format: 'HH:mm'
    },
    ...props.chartOptions
  }
})
</script>

<template>
  <div class="chart-container">
    <GChart :settings="settings" type="Timeline" :data="data" :options="options" />
  </div>
</template>

<style scoped>
.chart-container {
  width: 100%;
  max-width: 800px;
  padding: 10px;
  box-sizing: border-box;
}
</style>
