<script setup lang="ts">
import {
  type BusinessTask,
  getTask,
  nextTaskId,
  useBusinessTasks
} from '@/composables/TaskComposable'

const businessTasks = useBusinessTasks()

function onNewTask(task: BusinessTask) {
  businessTasks.value.push(task)

  for (const conflictId of task.cuttingInfo.conflicts) {
    const conflictTask = getTask(conflictId)!
    conflictTask.cuttingInfo.conflicts.push(task.id)
  }

  nextTaskId.value++
}

function deleteTask(id: number) {
  const task = getTask(id)!

  for (const conflictId of task.cuttingInfo.conflicts) {
    const conflictTask = getTask(conflictId)!
    conflictTask.cuttingInfo.conflicts = conflictTask.cuttingInfo.conflicts.filter(
      (conflictId) => conflictId !== id
    )
  }

  businessTasks.value = businessTasks.value.filter((task) => task.id !== id)
}
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center pt-5">
    <AddBusinessTaskComponent
      :businessTasks="businessTasks"
      class="mb-2"
      @onNewTask="onNewTask"
      :nextId="nextTaskId"
    />

    <div class="container overflow-auto min-px-100 p-0 border">
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
            <td>{{ task.cuttingInfo.processTime }}</td>
            <td>{{ task.cuttingInfo.weight }}</td>
            <td>
              {{
                task.cuttingInfo.conflicts
                  .map((conflictIndex) => getTask(conflictIndex)?.name)
                  .join(', ')
              }}
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
  </div>
</template>

<style scoped></style>
