<script setup lang="ts">
import { type BusinessTask, getTask, useBusinessTasks } from '@/composables/TaskComposable'
import { ref } from 'vue'

const businessTasks = useBusinessTasks()

const editingTask = ref<BusinessTask | undefined>()
const isEditing = ref<boolean>(false)

function edit(id: number) {
  editingTask.value = getTask(id)
  isEditing.value = true
}

function remove(id: number) {
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
    <BModal v-model="isEditing" hideFooter title="Edycja zadania">
      <BusinessTaskEditorComponent v-model="editingTask" />
    </BModal>
    <div class="container overflow-auto min-px-100 p-0 border">
      <table class="table table-dark table-bordered m-0">
        <thead>
          <tr>
            <th>Id</th>
            <th>Nazwa</th>
            <th>Wycinanie</th>
            <th>Szlifowanie</th>
            <th>Lakierowanie</th>
            <th>Waga</th>
            <th>Konflikty</th>
            <th></th>
          </tr>
        </thead>
        <tbody class="scrollable">
          <tr v-for="task in businessTasks" :key="task.name">
            <td>{{ task.id }}</td>
            <td>{{ task.name }}</td>
            <td>{{ task.cuttingInfo.processTime }} minut</td>
            <td>{{ task.flowInfo.grindingProcessTime }} minut</td>
            <td>{{ task.flowInfo.lacqueringProcessTime }} minut</td>
            <td>{{ task.cuttingInfo.weight }}</td>
            <td>
              {{
                task.cuttingInfo.conflicts
                  .map((conflictIndex) => getTask(conflictIndex)?.name)
                  .join(', ')
              }}
            </td>
            <td>
              <button class="btn btn-danger btn-sm m-1 float-end" @click="() => remove(task.id)">
                <TrashIconComponent />
              </button>
              <button class="btn btn-success btn-sm m-1 float-end" @click="() => edit(task.id)">
                <EditIconComponent />
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped></style>
