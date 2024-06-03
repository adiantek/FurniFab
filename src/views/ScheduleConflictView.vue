<script setup lang="ts">
import {
  ConflictAlgorithm,
  scheduleConflicts,
  type Conflict,
  type ConflictGraph,
  type ConflictTask,
  type Instance
} from '@/api'
import ScheduleComponent, { type ScheduledTask } from '@/components/ScheduleComponent.vue'
import {
  getTask,
  getTaskIndex,
  useBusinessTasks,
  type BusinessTask
} from '@/composables/TaskComposable'
import { plusMinutes } from '@/utils'
import { computed, ref } from 'vue'

const deadline = 8 * 60

const businessTasks = useBusinessTasks()

const processors = ref<number>(3)
const algorithm = ref<ConflictAlgorithm>(ConflictAlgorithm.List)

const todayTasks = computed<BusinessTask[]>(() =>
  businessTasks.value
    .filter((task) => task.cuttingInfo.startingTime !== undefined)
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
    if (info.startingTime !== undefined) {
      task.cuttingInfo.startingTime = undefined
      task.cuttingInfo.machine = undefined
    }
  }
}

const key = ref(0)
async function schedule() {
  let d = new Date(new Date().setHours(8, 0, 0, 0));
  while (notScheduledTasks.value.length) {
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
        info.startingTime = plusMinutes(d, scheduleInfo.start_time)
        info.machine = scheduleInfo.processor
      }
    })
    key.value++
    d.setDate(d.getDate() + 1);
  }
}
</script>

<template>
  <h4 v-if="!!notScheduledTasks.length" class="m-3">Zadania bez uszeregowania:</h4>
  <h4 v-else class="m-3">Wszystkie zadania są uszeregowane.</h4>
  <div v-if="!!notScheduledTasks.length" class="d-flex overflow-auto min-px-100 p-0 border w-100 my-2">
    <b-table-simple class="m-0" small bordered hover>
      <b-thead class="text-center">
        <b-tr>
          <b-th>Id</b-th>
          <b-th>Nazwa</b-th>
          <b-th>Czas wykonania</b-th>
          <b-th>Waga</b-th>
          <b-th>Konflikty</b-th>
        </b-tr>
      </b-thead>
      <b-tbody>
        <b-tr v-for="task in notScheduledTasks" :key="task.id">
          <b-td>{{ task.id }}</b-td>
          <b-td>{{ task.name }}</b-td>
          <b-td>{{ task.cuttingInfo.processTime }} minut</b-td>
          <b-td>{{ task.cuttingInfo.weight }}</b-td>
          <b-td>
            {{
              task.cuttingInfo.conflicts
                .map((conflictIndex) => getTask(conflictIndex)?.name)
                .join(', ')
            }}
          </b-td>
        </b-tr>
      </b-tbody>
    </b-table-simple>
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
      <LoadingButton @click="schedule" :disabled="!!mappedTasks.length || !notScheduledTasks.length">
        Utwórz uszeregowanie
      </LoadingButton>
      <button class="btn btn-primary m-auto" @click="reset" :disabled="!mappedTasks.length">
        Resetuj
      </button>
    </div>
  </div>

  <div class="border mt-2 p-2 rounded" v-if="score">
    Suma priorytetów uszeregowanych zadań: {{ score }}
  </div>

  <ScheduleComponent v-if="mappedTasks.length" :key="key" :tasks="mappedTasks" />
  <h4 v-else class="m-3">Brak zadań uszeregowanych tego dnia.</h4>
</template>
