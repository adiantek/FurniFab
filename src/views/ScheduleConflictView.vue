<script setup lang="ts">
import ScheduleComponent, { type ScheduledTask } from '@/components/ScheduleComponent.vue'
import { computed, ref } from 'vue'
import {
  type Conflict,
  type ConflictGraph,
  type Instance,
  scheduleConflicts,
  type ConflictTask,
  ConflictAlgorithm
} from '@/api'
import {
  type BusinessTask,
  getTask,
  getTaskIndex,
  useBusinessTasks
} from '@/composables/TaskComposable'
import { isSameDay, plusMinutes } from '@/utils'

const deadline = 8 * 60

const businessTasks = useBusinessTasks()

const processors = ref<number>(3)
const date = ref<Date>(new Date(new Date().setHours(8, 0, 0, 0)))
const algorithm = ref<ConflictAlgorithm>(ConflictAlgorithm.List)

const todayTasks = computed<BusinessTask[]>(() =>
  businessTasks.value
    .filter((task) => task.cuttingInfo.startingTime !== undefined)
    .filter((task) => isSameDay(date.value, task.cuttingInfo.startingTime!))
)
const mappedTasks = computed<ScheduledTask[]>(() =>
  todayTasks.value
    .map((task) => ({
      machine: task.cuttingInfo.machine!,
      name: task.name,
      start: task.cuttingInfo.startingTime!,
      end: plusMinutes(task.cuttingInfo.startingTime!, task.cuttingInfo.processTime)
    }))
    .sort((first, second) => first.machine - second.machine)
)
const notScheduledTasks = computed<BusinessTask[]>(() =>
  businessTasks.value.filter((task) => task.cuttingInfo.startingTime === undefined)
)
const score = computed<number>(() =>
  todayTasks.value.reduce((acc, task) => acc + task.cuttingInfo.weight, 0)
)

function reset() {
  for (let task of businessTasks.value) {
    const info = task.cuttingInfo
    if (info.startingTime !== undefined && isSameDay(date.value, info.startingTime)) {
      task.cuttingInfo.startingTime = undefined
      task.cuttingInfo.machine = undefined
    }
  }
}

async function schedule() {
  const tasks = [...notScheduledTasks.value]

  const apiTasks: ConflictTask[] = tasks.map((task) => ({
    processing_time: task.cuttingInfo.processTime,
    weight: task.cuttingInfo.weight
  }))

  const graph: ConflictGraph = tasks.flatMap((task, index) =>
    task.cuttingInfo.conflicts
      .filter((other) => other > task.id)
      .map((conflict) => getTaskIndex(tasks, conflict))
      .filter((conflict) => conflict !== -1)
      .map((conflict) => [index, conflict] as Conflict)
  )

  const instance: Instance = {
    deadline,
    processors: processors.value,
    tasks: apiTasks,
    graph
  }

  const schedule = await scheduleConflicts(instance, algorithm.value)

  schedule.schedule.forEach((scheduleInfo, index) => {
    if (scheduleInfo !== null) {
      const info = tasks[index].cuttingInfo
      info.startingTime = plusMinutes(date.value, scheduleInfo.start_time)
      info.machine = scheduleInfo.processor
    }
  })
}
</script>

<template>
  <h4 v-if="!!notScheduledTasks.length" class="m-3">Zadania bez uszeregowania:</h4>
  <h4 v-else class="m-3">Wszystkie zadania są uszeregowane.</h4>
  <div v-if="!!notScheduledTasks.length" class="d-flex overflow-auto min-px-100 p-0 border w-100 m-2">
    <table class="table table-dark table-bordered m-0">
      <thead>
        <tr>
          <th>Id</th>
          <th>Nazwa</th>
          <th>Czas wykonania</th>
          <th>Waga</th>
          <th>Konflikty</th>
        </tr>
      </thead>
      <tbody class="scrollable">
        <tr v-for="task in notScheduledTasks" :key="task.name">
          <td>{{ task.id }}</td>
          <td>{{ task.name }}</td>
          <td>{{ task.cuttingInfo.processTime }} minut</td>
          <td>{{ task.cuttingInfo.weight }}</td>
          <td>
            {{
              task.cuttingInfo.conflicts
                .map((conflictIndex) => getTask(conflictIndex)?.name)
                .join(', ')
            }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="card p-2">
    <div class="input-group mb-1">
      <label class="input-group-text">Liczba pracowników</label>
      <input v-model="processors" type="number" class="form-control" min="1" />
    </div>

    <div class="input-group mb-1">
      <label class="input-group-text">Algorytm</label>
      <select v-model="algorithm" class="form-select">
        <option :key="algo" v-for="algo in Object.keys(ConflictAlgorithm)" :value="algo">
          {{ algo }}
        </option>
      </select>
    </div>

    <div class="card-group mb-1">
      <button
        class="btn btn-primary m-auto"
        @click="schedule"
        :disabled="!!mappedTasks.length || !notScheduledTasks.length"
      >
        Utwórz uszeregowanie
      </button>
      <button class="btn btn-primary m-auto" @click="reset" :disabled="!mappedTasks.length">
        Resetuj
      </button>
    </div>

    <AdvancedDatePickerComponent v-model="date" />
  </div>

  <div class="border mt-2 p-2 rounded" v-if="score">
    Suma priorytetów uszeregowanych zadań: {{ score }}
  </div>

  <ScheduleComponent v-if="mappedTasks.length" :tasks="mappedTasks" />
  <h4 v-else class="m-3">Brak zadań uszeregowanych tego dnia.</h4>
</template>
