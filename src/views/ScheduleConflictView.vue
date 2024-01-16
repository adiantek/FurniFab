<script setup lang="ts">
import ScheduleComponent, { type ScheduledTask } from '@/components/ScheduleComponent.vue'
import { ref } from 'vue'

const tasks = ref<ScheduledTask[]>([])

tasks.value.push({
  machine: 1,
  name: 'task1',
  end: new Date(3600000),
  start: new Date(0)
})
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center loader">
    <ScheduleComponent :tasks="tasks" />
  </div>
</template>

<style scoped></style>
