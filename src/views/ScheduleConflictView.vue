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
import { type BusinessTask, useBusinessTasks } from '@/composables/TaskComposable'

const deadline = 4 * 60
const startDate = new Date()
startDate.setHours(8, 0, 0, 0)

const businessTasks = useBusinessTasks()

const processors = ref<number>(3)
const tasks = ref<ScheduledTask[]>([])
const tasksWithoutSchedule = ref<BusinessTask[]>([])
const score = ref<number | undefined>()

function getTaskIndex(id: number): number {
  return businessTasks.value.findIndex((task) => task.id === id)
}

function resetSchedule() {
  tasks.value = []
  tasksWithoutSchedule.value = []
  score.value = undefined
}

async function callBackend() {
  tasks.value = []
  tasksWithoutSchedule.value = []

  const apiTasks: Task[] = businessTasks.value.map((task) => ({
    processing_time: task.cuttingInfo.processTime,
    weight: task.cuttingInfo.weight
  }))

  const graph: ConflictGraph = businessTasks.value.flatMap((task, index) =>
    task.cuttingInfo.conflicts
      .filter((other) => other > task.id)
      .map((conflictId) => [index, getTaskIndex(conflictId)] as Conflict)
  )

  const instance: Instance = {
    deadline,
    processors: processors.value,
    tasks: apiTasks,
    graph
  }

  let [schedule, newScore] = await scheduleWithConflicts(instance)

  score.value = newScore

  schedule.schedule.forEach((scheduleInfo, index) => {
    const task = businessTasks.value[index]
    if (scheduleInfo) {
      tasks.value.push({
        machine: scheduleInfo.processor,
        name: task.name,
        start: new Date(startDate.getTime() + scheduleInfo.start_time * 60000),
        end: new Date(
          startDate.getTime() + (scheduleInfo.start_time + task.cuttingInfo.processTime) * 60000
        )
      })
    } else {
      tasksWithoutSchedule.value.push(task)
    }
  })

  tasks.value.sort((a, b) => a.machine - b.machine)
}
</script>

<template>
  <div
    class="h-100 w-100 d-flex flex-column align-items-center justify-content-center pt-5 overflow-auto"
  >
    <div class="mt-3">
      <label for="processors" class="form-label">Liczba pracowników: </label>
      <input v-model="processors" name="processors" type="number" class="form-control mb-1" />
    </div>
    <div class="mt-3">
      <button class="btn btn-primary m-1" @click="callBackend">Utwórz uszeregowanie</button>
      <button class="btn btn-primary m-1" @click="resetSchedule">Resetuj</button>
    </div>
    <div v-if="tasksWithoutSchedule.length">
      Zadanie nie objęte uszeregowaniem:
      <ul>
        <li v-for="task in tasksWithoutSchedule" :key="task.name">
          {{ task.name }}
        </li>
      </ul>
    </div>
    <div>
      <span v-if="score">Wynik: {{ score }}</span>
    </div>
    <ScheduleComponent v-if="tasks.length" :tasks="tasks" />
  </div>
</template>

<style scoped></style>
