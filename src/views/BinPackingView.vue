<script setup lang="ts">
import { BinPackingAlgorithm, binPacking } from '@/api';
import {
  useBusinessTasks,
  type BusinessTask,
  type RectInfo,
  useBoardWidth,
  useBoardHeight
} from '@/composables/TaskComposable'
import { computed, ref } from 'vue';

const algorithm = ref(BinPackingAlgorithm.FFDH);
const businessTasks = computed(() => {
  return useBusinessTasks().value.filter((task) => task.rectInfo !== undefined);
});
const binW = useBoardWidth();
const binH = useBoardHeight();

const runningAlgo = ref(false);
const runAlgo = async () => {
  runningAlgo.value = true;
  try {
    for (const task of businessTasks.value) {
      task.rectInfo.x = -1;
      task.rectInfo.y = -1;
      task.rectInfo.bin_id = -1;
      if (task.rectInfo.w < 1) {
        task.rectInfo.w = 1;
      }
      if (task.rectInfo.h < 1) {
        task.rectInfo.h = 1;
      }
    }
    const rectangles: RectInfo[] = businessTasks.value.map((task: BusinessTask) => {
      return {
        id: task.id,
        w: task.rectInfo.w,
        h: task.rectInfo.h,
      };
    });
    const result = await binPacking(
      { id: 1, w: binW.value, h: binH.value },
      rectangles,
      algorithm.value
    );
    for (let i = 0; i < result.length; i++) {
      result[i].color = `hsl(${(result[i].id * 50) % 360}, 100%, 50%)`;
    }
    const map = new Map<number, RectInfo>();
    for (const rect of result) {
      map.set(rect.id, rect);
    }
    for (const task of businessTasks.value) {
      if (map.has(task.id)) {
        task.rectInfo = map.get(task.id)!;
      } else {
        task.rectInfo.x = -1;
        task.rectInfo.y = -1;
      }
    }
  } finally {
    runningAlgo.value = false;
  }
};
</script>
<style scoped>
.max-h-100 {
  max-height: 100%;
}

.colored-rect {
  display: inline-block;
  width: 1rem;
  height: 1rem;
  margin-right: 0.5rem;
}
</style>
<template>
  <b-row class="w-100 h-100">
    <b-col class="d-flex flex-column max-h-100">
      <b-form-floating-label label="Wybierz algorytm" class="mb-3">
        <b-form-select v-model="algorithm" :options="Object.values(BinPackingAlgorithm)" />
      </b-form-floating-label>
      <b-row gutter-x="3">
        <b-col sm="12" md="6">
          <b-form-floating-label label="Szerokość płyty" class="mb-3">
            <b-form-input type="number" v-model.number="binW" placeholder="" />
          </b-form-floating-label>
        </b-col>
        <b-col sm="12" md="6">
          <b-form-floating-label label="Wysokość płyty" class="mb-3">
            <b-form-input type="number" v-model.number="binH" placeholder="" />
          </b-form-floating-label>
        </b-col>
      </b-row>
      <div class="overflow-x-auto flex-grow-1">
        <b-table-simple bordered small>
          <b-thead>
            <b-tr class="text-center">
              <b-th>#</b-th>
              <b-th>X</b-th>
              <b-th>Y</b-th>
              <b-th>W</b-th>
              <b-th>H</b-th>
            </b-tr>
          </b-thead>
          <b-tbody>
            <b-tr v-for="task of businessTasks" :key="task.id" :class="{ 'table-active': task.rectInfo.mouseOver }">
              <b-td class="text-center">
                <div class="d-flex align-items-center">
                  <span class="colored-rect" :style="`background: ${task.rectInfo.color}`"
                    :id="`tooltip-${task.id}`"></span>
                  {{ task.id }}
                  <b-tooltip triggers="hover" :target="`tooltip-${task.id}`">{{ task.name }}</b-tooltip>
                </div>
              </b-td>
              <b-td colspan="2" class="px-2 text-center text-nowrap" v-if="task.rectInfo.x === -1">
                Nie mieści się
              </b-td>
              <template v-else>
                <b-td class="px-2 text-end">{{ task.rectInfo.x ?? "-" }}</b-td>
                <b-td class="px-2 text-end">{{ task.rectInfo.y ?? "-" }}</b-td>
              </template>
              <b-td>
                <b-form-input type="number" v-model.number="task.rectInfo.w" placeholder="" />
              </b-td>
              <b-td>
                <b-form-input type="number" v-model.number="task.rectInfo.h" placeholder="" />
              </b-td>
            </b-tr>
          </b-tbody>
        </b-table-simple>
      </div>
      <b-button variant="primary" @click="runAlgo" class="d-block mx-auto"
        :disabled="runningAlgo || businessTasks.length === 0">Utwórz
        ułożenie</b-button>
    </b-col>
    <b-col class="overflow-auto">
      <svg class="w-100 h-100" :viewBox="`0 0 ${binW} ${binH}`">
        <rect :width="binW" :height="binH" fill="#40404040" />
        <template v-for="task of businessTasks" :key="task.id">
          <rect v-if="task.rectInfo.x !== -1 && task.rectInfo.x !== undefined" :width="task.rectInfo.w"
            :height="task.rectInfo.h" :x="task.rectInfo.x" :y="task.rectInfo.y" :fill="task.rectInfo.color"
            @mouseover="task.rectInfo.mouseOver = true" @mouseout="task.rectInfo.mouseOver = undefined" />
        </template>
      </svg>
    </b-col>
  </b-row>
</template>