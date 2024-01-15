<script setup lang="ts">
import { GChart } from 'vue-google-charts'
import { ref } from 'vue'
import type { GoogleChartOptions } from 'vue-google-charts/dist/types'

interface Task {
  machine: number
  name: string
  start: Date
  end: Date
}

const data = ref([['Role', 'Name', 'Start', 'End']])

function addTask(task: Task): void {
  data.value.push([`Machine ${task.machine}`, task.name, task.start, task.end])
}

addTask({
  machine: 1,
  name: 'Task 1',
  start: new Date(2021, 9, 1, 8, 0, 0),
  end: new Date(2021, 9, 1, 10, 0, 0)
})

addTask({
  machine: 2,
  name: 'Task 2',
  start: new Date(2021, 9, 1, 8, 0, 0),
  end: new Date(2021, 9, 1, 10, 0, 0)
})

addTask({
  machine: 3,
  name: 'Task 3',
  start: new Date(2021, 9, 1, 8, 0, 0),
  end: new Date(2021, 9, 1, 10, 0, 0)
})

addTask({
  machine: 1,
  name: 'Task 4',
  start: new Date(2021, 9, 1, 10, 0, 0),
  end: new Date(2021, 9, 1, 12, 0, 0)
})

addTask({
  machine: 5,
  name: 'Task 2',
  start: new Date(2021, 9, 1, 8, 0, 0),
  end: new Date(2021, 9, 1, 10, 0, 0)
})

const options = ref<GoogleChartOptions>({
  width: 600,
  backgroundColor: '#2c2d28',
})


window.addEventListener('resize', () => {
  options.value = {
    ...options.value,
    width: Math.min(window.innerWidth - 100, 1000)
  }
})

</script>

<template>
  <div>
    <div style="color: black; padding: 10px;">
      <GChart :settings="{ packages: ['table'] }" type="Table" :data="data" :options="options" />
    </div>

    <div style="color: white; padding: 10px;">
      <GChart
        style="background-color: white"
        :settings="{ packages: ['timeline'] }"
        type="Timeline"
        :data="data"
        :options="options"
      />
    </div>
  </div>
</template>

<style scoped></style>
