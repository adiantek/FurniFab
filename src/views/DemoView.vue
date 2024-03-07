<script setup lang="ts">
import { ref } from 'vue'
import { binPacking, BinPackingAlgorithm, type Rect } from '@/api'

const result = ref<Rect[] | undefined>()

async function onButtonClick() {
  console.log('start');
  result.value = await binPacking(
    { id: 1, w: 100, h: 100 },
    [
      { id: 2, w: 10, h: 10 },
      { id: 3, w: 10, h: 10 },
      { id: 4, w: 10, h: 10 },
      { id: 5, w: 10, h: 10 },
      { id: 6, w: 10, h: 10 },
      { id: 7, w: 10, h: 10 },
      { id: 8, w: 10, h: 10 },
      { id: 9, w: 10, h: 10 },
      { id: 10, w: 10, h: 10 },
      { id: 11, w: 10, h: 10 },
      { id: 12, w: 10, h: 10 },
      { id: 13, w: 10, h: 10 },
      { id: 14, w: 10, h: 10 },
      { id: 15, w: 10, h: 10 },
      { id: 16, w: 10, h: 10 },
      { id: 17, w: 10, h: 10 },
      { id: 18, w: 10, h: 10 },
      { id: 19, w: 10, h: 10 },
      { id: 20, w: 10, h: 10 }
    ],
    BinPackingAlgorithm.FFDH
  )
  console.log('end');
  console.log(result.value);
}
</script>

<template>
  <div class="h-100 w-100 d-flex flex-column align-items-center justify-content-center">
    <button @click="onButtonClick">Run bin packing demo</button>
    <div class="overflow-scroll m-2">
      <div v-for="rect in result" :key="rect.id">
        <div class="d-inline-block">X: {{ rect.x }}</div>
        <div class="d-inline-block">Y: {{ rect.y }}</div>
        <div class="d-inline-block">W: {{ rect.w }}</div>
        <div class="d-inline-block">H: {{ rect.h }}</div>
      </div>
    </div>
  </div>
</template>