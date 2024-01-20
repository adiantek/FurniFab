<script setup lang="ts">
import { computed, ref } from 'vue'
import type { BusinessTask } from '@/composables/TaskComposable'
import { BModal } from 'bootstrap-vue-next'

const callback = defineEmits<{
  onNewTask: [BusinessTask]
}>()

const props = defineProps<{
  businessTasks: BusinessTask[]
  nextId: number
}>()

const showingModal = ref<boolean>(false)
const name = ref<string | undefined>()
const processTime = ref<number | undefined>()
const weight = ref<number | undefined>()
const conflicts = ref<number[]>([])

const options = computed(() =>
  props.businessTasks.map((task) => {
    return { value: task.id, text: task.name }
  })
)

function save() {
  callback('onNewTask', {
    id: props.nextId,
    name: name.value!,
    cuttingInfo: {
      processTime: processTime.value!,
      weight: weight.value!,
      conflicts: conflicts.value
    }
  })

  showingModal.value = false
  name.value = undefined
  processTime.value = undefined
  weight.value = undefined
  conflicts.value = []
}

const validTask = computed(() => {
  return name.value && processTime.value && weight.value && name.value.length > 0
})
</script>

<template>
  <div>
    <button class="btn btn-primary" @click="showingModal = !showingModal">Dodaj zadanie</button>
    <BModal
      v-model="showingModal"
      @ok="save"
      :okDisabled="!validTask"
      okOnly
      okTitle="Dodaj"
      title="Dodaj zadanie"
    >
      <div class="form-group">
        <input v-model="name" type="text" placeholder="Nazwa" class="form-control mb-1" />
        <input
          v-model="processTime"
          type="number"
          placeholder="Czas trwania"
          class="form-control mb-1"
        />
        <input v-model="weight" type="number" placeholder="Waga" class="form-control mb-1" />
        <label class="m-1">Konflikty: </label>
        <BFormSelect v-model="conflicts" :options="options" multiple />
      </div>
    </BModal>
  </div>
</template>

<style scoped></style>
