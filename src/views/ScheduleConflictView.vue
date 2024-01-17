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
import type { BusinessTask } from '@/interface'

const deadline = 4 * 60
const startDate = new Date()
startDate.setHours(8, 0, 0, 0)

const processors = ref<number>(3)
const businessTasks = ref<BusinessTask[]>([
  { id: 0, name: 'Zadanie 0', processTime: 75, weight: 8, conflicts: [5, 1] },
  { id: 1, name: 'Zadanie 1', processTime: 60, weight: 1, conflicts: [0, 6, 7] },
  { id: 2, name: 'Zadanie 2', processTime: 70, weight: 4, conflicts: [3, 4, 7] },
  { id: 3, name: 'Zadanie 3', processTime: 40, weight: 3, conflicts: [2, 7] },
  { id: 4, name: 'Zadanie 4', processTime: 65, weight: 7, conflicts: [2, 5, 9] },
  { id: 5, name: 'Zadanie 5', processTime: 100, weight: 11, conflicts: [0, 4, 9] },
  { id: 6, name: 'Zadanie 6', processTime: 30, weight: 2, conflicts: [1] },
  { id: 7, name: 'Zadanie 7', processTime: 120, weight: 5, conflicts: [1, 2, 3] },
  { id: 8, name: 'Zadanie 8', processTime: 40, weight: 1, conflicts: [9] },
  { id: 9, name: 'Zadanie 9', processTime: 75, weight: 3, conflicts: [4, 5, 8] }
])
const nextTaskId = ref<number>(10)
const tasks = ref<ScheduledTask[]>([])
const tasksWithoutSchedule = ref<BusinessTask[]>([])
const score = ref<number | undefined>()

function getTaskIndex(id: number): number {
  return businessTasks.value.findIndex((task) => task.id === id)
}

function getTask(id: number): BusinessTask | undefined {
  return businessTasks.value.find((task) => task.id === id)
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
    processing_time: task.processTime,
    weight: task.weight
  }))

  const graph: ConflictGraph = businessTasks.value.flatMap((task, index) =>
    task.conflicts
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
        end: new Date(startDate.getTime() + (scheduleInfo.start_time + task.processTime) * 60000)
      })
    } else {
      tasksWithoutSchedule.value.push(task)
    }
  })

  tasks.value.sort((a, b) => a.machine - b.machine)
}

function onNewTask(task: BusinessTask) {
  businessTasks.value.push(task)

  for (const conflictId of task.conflicts) {
    const conflictTask = getTask(conflictId)!
    conflictTask.conflicts.push(task.id)
  }

  nextTaskId.value++
  resetSchedule()
}

function deleteTask(id: number) {
  const task = getTask(id)!

  for (const conflictId of task.conflicts) {
    const conflictTask = getTask(conflictId)!
    conflictTask.conflicts = conflictTask.conflicts.filter((conflictId) => conflictId !== id)
  }

  businessTasks.value = businessTasks.value.filter((task) => task.id !== id)
  resetSchedule()
}
</script>

<template>
  <div
    class="h-100 w-100 d-flex flex-column align-items-center justify-content-center pt-5 overflow-auto"
  >
    <AddBusinessTaskComponent
      :businessTasks="businessTasks"
      class="mb-2"
      @onNewTask="onNewTask"
      :nextId="nextTaskId"
    />

    <div class="container overflow-auto w-75 min-px-100 p-0 border">
      <table class="table table-dark table-bordered table-sm m-0">
        <thead>
          <tr>
            <th>Id</th>
            <th>Nazwa</th>
            <th>Czas trwania</th>
            <th>Waga</th>
            <th>Konflikty</th>
            <th></th>
          </tr>
        </thead>
        <tbody class="scrollable">
          <tr v-for="task in businessTasks" :key="task.name">
            <td>{{ task.id }}</td>
            <td>{{ task.name }}</td>
            <td>{{ task.processTime }}</td>
            <td>{{ task.weight }}</td>
            <td>
              {{ task.conflicts.map((conflictIndex) => getTask(conflictIndex)?.name).join(', ') }}
            </td>
            <td>
              <button
                class="btn btn-danger btn-sm m-1 float-end"
                @click="() => deleteTask(task.id)"
              >
                <TrashIconComponent />
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="mt-3">
      <label for="processors" class="form-label">Liczba procesorów: </label>
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
