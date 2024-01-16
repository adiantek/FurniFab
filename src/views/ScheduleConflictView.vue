<script setup lang="ts">
import ScheduleComponent, { type ScheduledTask } from '@/components/ScheduleComponent.vue'
import { ref } from 'vue'
import {
  type Conflict,
  type ConflictGraph,
  type Instance,
  scheduleWithConflicts,
  type Task
} from '@/api'

interface BusinessTask {
  name: string
  processTime: number
  weight: number
  conflicts: number[]
}

// TODO: hardcoded tasks
const businessTasks: BusinessTask[] = [
  { name: 'task0', processTime: 75, weight: 8, conflicts: [5, 1] },
  { name: 'task1', processTime: 60, weight: 1, conflicts: [0, 6, 7] },
  { name: 'task2', processTime: 70, weight: 4, conflicts: [3, 4, 7] },
  { name: 'task3', processTime: 40, weight: 3, conflicts: [2, 7] },
  { name: 'task4', processTime: 65, weight: 7, conflicts: [2, 5, 9] },
  { name: 'task5', processTime: 100, weight: 11, conflicts: [0, 4, 9] },
  { name: 'task6', processTime: 30, weight: 2, conflicts: [1] },
  { name: 'task7', processTime: 120, weight: 5, conflicts: [1, 2, 3] },
  { name: 'task8', processTime: 40, weight: 1, conflicts: [9] },
  { name: 'task9', processTime: 75, weight: 3, conflicts: [4, 5, 8] }
]

// TODO: hardcoded deadline
const deadline = 4 * 60
// TODO: hardcoded processors
const processors = 3
// TODO: hardcoded start time
const startDate = new Date()
startDate.setDate(startDate.getDate() + 1)
startDate.setHours(8, 0, 0, 0)

const tasks = ref<ScheduledTask[]>([])
const tasksWithoutSchedule = ref<BusinessTask[]>([])
const score = ref<number | undefined>()

async function callBackend() {
  tasks.value = []
  tasksWithoutSchedule.value = []

  const apiTasks: Task[] = businessTasks.map((task) => ({
    processing_time: task.processTime,
    weight: task.weight
  }))

  const graph: ConflictGraph = businessTasks.flatMap((task, index) =>
    task.conflicts.filter((other) => other > index).map((conflict) => [index, conflict] as Conflict)
  )

  const instance: Instance = {
    deadline,
    processors,
    tasks: apiTasks,
    graph
  }

  let [schedule, newScore] = await scheduleWithConflicts(instance)

  score.value = newScore

  schedule.schedule.forEach((scheduleInfo, index) => {
    if (scheduleInfo) {
      tasks.value.push({
        machine: scheduleInfo.processor,
        name: businessTasks[index].name,
        start: new Date(startDate.getTime() + scheduleInfo.start_time * 60000),
        end: new Date(
          startDate.getTime() +
            scheduleInfo.start_time * 60000 +
            businessTasks[index].processTime * 60000
        )
      })
    } else {
      tasksWithoutSchedule.value.push(businessTasks[index])
    }
  })

  tasks.value.sort((a, b) => a.machine - b.machine)
}
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center loader">
    <div class="mt-3">
      <button class="btn btn-primary" @click="callBackend">Schedule</button>
    </div>
    <ScheduleComponent v-if="tasks.length" :tasks="tasks" />
    <div>
      <div v-if="tasksWithoutSchedule.length">
        Tasks not scheduled:
        <ul>
          <li v-for="task in tasksWithoutSchedule" :key="task.name">
            {{ task.name }}
          </li>
        </ul>
      </div>
    </div>
    <div class="mt-3">
      <span v-if="score">Score: {{ score }}</span>
    </div>
  </div>
</template>

<style scoped></style>
