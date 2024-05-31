<script setup lang="ts">
import { FlowScript, scheduleFlow, type FlowScheduleInfo, type FlowTask } from '@/api'
import type { ScheduledTask } from '@/components/ScheduleComponent.vue'
import { useBusinessTasks, type BusinessTask } from '@/composables/TaskComposable'
import { plusMinutes } from '@/utils'
import { computed, ref } from 'vue'

const businessTasks = useBusinessTasks()

const script = ref<FlowScript>(FlowScript.Pa)

const readyTasks = computed<BusinessTask[]>(() =>
  businessTasks.value
    .filter((task) => task.cuttingInfo.startingTime !== undefined)
    .filter((task) => task.flowInfo.grinding === undefined)
)

const scheduledTasks = computed<BusinessTask[]>(() =>
  businessTasks.value.filter((task) => task.flowInfo.grinding !== undefined)
)

const mappedTasks = computed<ScheduledTask[]>(() =>
  scheduledTasks.value
    .flatMap((task) => {
      const result = []
      for (let grinding of task.flowInfo.grinding!) {
        let [start, duration] = grinding

        result.push({
          machine: 0,
          name: task.name,
          start: start,
          end: plusMinutes(start, duration)
        })
      }

      for (let lacquering of task.flowInfo.lacquering!) {
        let [start, duration] = lacquering

        result.push({
          machine: 1,
          name: task.name,
          start: start,
          end: plusMinutes(start, duration)
        })
      }
      return result
    })
    .sort((first, second) => first.machine - second.machine)
)

function reset() {
  for (let task of businessTasks.value) {
    const info = task.flowInfo
    info.grinding = undefined
    info.lacquering = undefined
  }
}

function getStartingTime(minDate: Date, task: BusinessTask): number {
  const min = new Date(minDate)
  min.setHours(8, 0, 0, 0)
  const startDate = new Date(task.cuttingInfo.startingTime!)
  startDate.setHours(8, 0, 0, 0)

  const dayDiff = Math.floor(new Date(startDate.getTime() - min.getTime()).getDate()) - 1

  return (
    Math.round(
      (task.cuttingInfo.startingTime!.getTime() - minDate.getTime()) / 60000 - dayDiff * 16 * 60
    ) + task.cuttingInfo.processTime
  )
}

function mergeTasks(tasks: FlowScheduleInfo[][]): FlowScheduleInfo[][] {
  return tasks.map((task) => {
    const newTask = []
    let previous = undefined

    task.sort((first, second) => first.start_time - second.start_time)
    for (let time of task) {
      if (previous !== undefined && previous.end_time === time.start_time) {
        previous.end_time = time.end_time
      } else {
        newTask.push(time)
        previous = time
      }
    }

    return newTask
  })
}

const key = ref(0)

async function schedule() {
  const tasks = [...readyTasks.value]

  const minDate = tasks.reduce((acc, task) => {
    if (task.cuttingInfo.startingTime! < acc) {
      return task.cuttingInfo.startingTime!
    }
    return acc
  }, tasks[0].cuttingInfo.startingTime!)

  const apiTasks: FlowTask[] = tasks.map((task) => ({
    start_time: getStartingTime(minDate, task),
    grinding_time: task.flowInfo.grindingProcessTime,
    lacquering_time: task.flowInfo.lacqueringProcessTime
  }))

  const schedule = await scheduleFlow(apiTasks, script.value)

  mergeTasks(schedule.grinding).forEach((times, index) => {
    const task = tasks[index]
    task.flowInfo.grinding = []
    for (let time of times) {
      let startTime = plusMinutes(minDate, time.start_time % 480)
      startTime.setDate(startTime.getDate() + Math.floor(time.start_time / 480))

      let endTime = plusMinutes(startTime, time.end_time - time.start_time)

      if (endTime.getHours() >= 16 && endTime.getMinutes() > 0) {
        let shiftEnd = new Date(endTime)
        shiftEnd.setHours(16, 0, 0, 0)

        const diff = (endTime.getTime() - shiftEnd.getTime()) / 60000

        task.flowInfo.grinding!.push([startTime, time.end_time - time.start_time - diff])

        let shiftStart = new Date(shiftEnd)
        shiftStart.setDate(shiftStart.getDate() + 1)
        shiftStart.setHours(8, 0, 0, 0)

        task.flowInfo.grinding!.push([shiftStart, diff])
      } else {
        task.flowInfo.grinding!.push([startTime, time.end_time - time.start_time])
      }
    }
  })

  mergeTasks(schedule.lacquering).forEach((times, index) => {
    const task = tasks[index]
    task.flowInfo.lacquering = []
    for (let time of times) {
      let startTime = plusMinutes(minDate, time.start_time % 480)
      startTime.setDate(startTime.getDate() + Math.floor(time.start_time / 480))

      let endTime = plusMinutes(startTime, time.end_time - time.start_time)

      if (endTime.getHours() >= 16 && endTime.getMinutes() > 0) {
        let shiftEnd = new Date(endTime)
        shiftEnd.setHours(16, 0, 0, 0)

        const diff = (endTime.getTime() - shiftEnd.getTime()) / 60000

        task.flowInfo.lacquering!.push([startTime, time.end_time - time.start_time - diff])

        let shiftStart = new Date(shiftEnd)
        shiftStart.setDate(shiftStart.getDate() + 1)
        shiftStart.setHours(8, 0, 0, 0)

        task.flowInfo.lacquering!.push([shiftStart, diff])
      } else {
        task.flowInfo.lacquering!.push([startTime, time.end_time - time.start_time])
      }
    }
  })
  key.value++
}
</script>

<template>
  <h4 v-if="!!readyTasks.length" class="m-3">Zadania gotowe do uszeregowania:</h4>
  <h4 v-else class="m-3">Brak zadań gotowych do uszeregowania.</h4>
  <div v-if="!!readyTasks.length" class="d-flex overflow-auto min-px-100 p-0 border w-100 m-2">
    <table class="table table-dark table-bordered m-0">
      <thead>
        <tr>
          <th>Id</th>
          <th>Nazwa</th>
          <th>Czas szlifowania</th>
          <th>Czas lakierowanie</th>
        </tr>
      </thead>
      <tbody class="scrollable">
        <tr v-for="task in readyTasks" :key="task.name">
          <td>{{ task.id }}</td>
          <td>{{ task.name }}</td>
          <td>{{ task.flowInfo.grindingProcessTime }} minut</td>
          <td>{{ task.flowInfo.lacqueringProcessTime }} minut</td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="card p-2">
    <div class="input-group mb-1">
      <label class="input-group-text">Algorytm</label>
      <select v-model="script" class="form-select">
        <option :key="script" v-for="script in Object.keys(FlowScript)" :value="script">
          {{ script }}
        </option>
      </select>
    </div>

    <div class="card-group mb-1">
      <LoadingButton @click="schedule" :disabled="!!scheduledTasks.length || !readyTasks.length">
        Utwórz uszeregowanie
      </LoadingButton>
      <button class="btn btn-primary m-auto" @click="reset" :disabled="!scheduledTasks.length">
        Resetuj
      </button>
    </div>
  </div>

  <ScheduleComponent
    v-if="mappedTasks.length"
    :tasks="mappedTasks"
    :key="key"
    :machine-name-formatter="(machine: number) => (machine == 0 ? 'Szlifowanie' : 'Lakierowanie')"
  />
  <h4 v-else class="m-3">Brak zadań uszeregowanych tego dnia.</h4>
</template>
