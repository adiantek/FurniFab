<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { binPacking, BinPackingAlgorithm, type Rect } from '@/api'

const result = ref<Rect[] | undefined>()

const algorithm = ref(BinPackingAlgorithm.FFDH);
const rectangles = ref([]) as Ref<Rect[]>;
const binW = ref(100);
const binH = ref(100);

const minW = ref(1);
const maxW = ref(10);
const minH = ref(1);
const maxH = ref(10);
const sizeRandPrompt = ref(false);

const addRect = () => {
  const w = Math.floor(Math.random() * (maxW.value - minW.value + 1) + minW.value);
  const h = Math.floor(Math.random() * (maxH.value - minH.value + 1) + minH.value);
  const maxID = rectangles.value.reduce((acc: number, curr: Rect) => Math.max(acc, curr.id), 0);
  rectangles.value.push({
    id: maxID + 1,
    w: w,
    h: h
  })
};

const randAll = () => {
  for (let i = 0; i < rectangles.value.length; i++) {
    rectangles.value[i].w = Math.floor(Math.random() * (maxW.value - minW.value + 1) + minW.value);
    rectangles.value[i].h = Math.floor(Math.random() * (maxH.value - minH.value + 1) + minH.value);
  }
  sizeRandPrompt.value = false;
};

const removeRect = (rect: Rect) => {
  const index = rectangles.value.indexOf(rect);
  if (index > -1) {
    rectangles.value.splice(index, 1);
  }
};

const runningAlgo = ref(false);
const runAlgo = async () => {
  runningAlgo.value = true;
  try {
    for (let i = 0; i < rectangles.value.length; i++) {
      console.log(rectangles.value);
      delete rectangles.value[i].x;
      delete rectangles.value[i].y;
      delete rectangles.value[i].bin_id;
    }
    const result = await binPacking(
      { id: 1, w: binW.value, h: binH.value },
      rectangles.value,
      algorithm.value
    );
    result.sort((a, b) => a.id - b.id);
    for (let i = 0; i < result.length; i++) {
      result[i].color = `hsl(${(result[i].id * 50) % 360}, 100%, 50%)`;
    }
    rectangles.value = result;
  } finally {
    runningAlgo.value = false;
  }
};
</script>
<style scoped>
.max-h-100 {
  max-height: 100%;
}
</style>
<template>
  <b-modal v-model="sizeRandPrompt" title="Wybierz przedział losowania">
    <b-form-floating-label label="Minimalna szerokość" class="mb-3">
      <b-form-input type="number" v-model.number="minW" placeholder="" />
    </b-form-floating-label>
    <b-form-floating-label label="Maksymalna szerokość" class="mb-3">
      <b-form-input type="number" v-model.number="maxW" placeholder="" />
    </b-form-floating-label>
    <b-form-floating-label label="Minimalna wysokość" class="mb-3">
      <b-form-input type="number" v-model.number="minH" placeholder="" />
    </b-form-floating-label>
    <b-form-floating-label label="Maksymalna wysokość" class="mb-3">
      <b-form-input type="number" v-model.number="maxH" placeholder="" />
    </b-form-floating-label>
    <template #footer>
      <b-button variant="secondary" @click="sizeRandPrompt = false">Zamknij</b-button>
      <b-button variant="primary" @click="randAll">Wylosuj</b-button>
    </template>
  </b-modal>
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
              <b-th></b-th>
            </b-tr>
          </b-thead>
          <b-tbody>
            <b-tr v-for="rect of rectangles" :key="rect.id">
              <b-td class="text-center">{{ rect.id }}</b-td>
              <b-td colspan="2" class="px-2 text-center text-nowrap" v-if="rect.x === -1">
                Nie mieści się
              </b-td>
              <template v-else>
                <b-td class="px-2 text-end">{{ rect.x ?? "-" }}</b-td>
                <b-td class="px-2 text-end">{{ rect.y ?? "-" }}</b-td>
              </template>
              <b-td class="w-number" >
                <b-form-input type="number" v-model.number="rect.w" placeholder="" />
              </b-td>
              <b-td class="w-number" >
                <b-form-input type="number" v-model.number="rect.h" placeholder="" />
              </b-td>
              <b-td>
                <b-button variant="danger" @click="removeRect(rect)">Usuń</b-button>
              </b-td>
            </b-tr>
          </b-tbody>
        </b-table-simple>
      </div>
      <b-row>
        <b-col sm="12" md="4" class="mt-3">
          <b-button variant="primary" @click="addRect" class="d-block mx-auto h-100">Dodaj element</b-button>
        </b-col>
        <b-col sm="12" md="4" class="mt-3">
          <b-button variant="primary" @click="sizeRandPrompt = true" class="d-block mx-auto h-100">Wylosuj rozmiary</b-button>
        </b-col>
        <b-col sm="12" md="4" class="mt-3">
          <b-button variant="primary" @click="runAlgo" class="d-block mx-auto h-100" :disabled="runningAlgo">Utwórz ułożenie</b-button>
        </b-col>
      </b-row>
    </b-col>
    <b-col class="overflow-auto">
      <svg class="w-100 h-100" :viewBox="`0 0 ${binW} ${binH}`">
        <template v-for="rect of rectangles" :key="rect.id">
          <rect v-if="rect.x !== -1 && rect.x !== undefined"  :width="rect.w" :height="rect.h" :x="rect.x" :y="rect.y" :fill="rect.color" />
        </template>
      </svg>
    </b-col>
  </b-row>
</template>