<script setup lang="ts">
import { findMaxFlowMinCost, type Edge } from '@/api'
import { deliveryCounter, transportCounter, useSuppliers } from '@/composables/SupplierComposable'
import { onMounted, onUnmounted, ref } from 'vue'

export type Line = {
  id: number
  group: number
  color: string

  x1: number
  y1: number
  x2: number
  y2: number

  angle: number

  p1: string
  p2: string

  cost: number
  flow?: number
  maxFlow: number
}

const { lines, deliveries, transports, names } = useSuppliers()

const addLine = (p1: string, p2: string, group: number) => {
  lines.value.push({
    id: lines.value.length,
    group,
    color: ['#dc3545', '#198754', '#0dcaf0'][group % 3],
    x1: 0,
    y1: 0,
    x2: 0,
    y2: 0,
    angle: 0,
    p1,
    p2,
    cost: 0,
    maxFlow: 0
  })
}

const sortLines = () => {
  lines.value.sort((a, b) => {
    if (a.group < b.group) {
      return -1
    }
    if (a.group > b.group) {
      return 1
    }
    const v = a.p1.localeCompare(b.p1)
    if (v !== 0) {
      return v
    }
    return a.p2.localeCompare(b.p2)
  })
}

const addDelivery = () => {
  const newPoint = deliveryCounter.value++
  names.value[`delivery-${newPoint}`] = `Dostawca nr ${newPoint}`
  deliveries.value.push(newPoint)
  addLine('startPoint', `delivery-${newPoint}`, 0)
  for (const transport of transports.value) {
    addLine(`delivery-${newPoint}`, `transport-${transport}`, 1)
  }
  sortLines()
}

const addTransport = () => {
  const newPoint = transportCounter.value++
  names.value[`transport-${newPoint}`] = `Towar nr ${newPoint}`
  transports.value.push(newPoint)
  for (const delivery of deliveries.value) {
    addLine(`delivery-${delivery}`, `transport-${newPoint}`, 1)
  }
  addLine(`transport-${newPoint}`, `endPoint`, 2)
  sortLines()
}

const svg = ref()
const graph = ref()

const updateSize = () => {
  const svgElement: SVGElement | undefined = svg.value
  if (!svgElement || !graph.value) {
    return
  }
  const graphBB = graph.value.$el.getBoundingClientRect()
  const svgBB = svgElement.getBoundingClientRect()

  svgElement.setAttribute('width', `${graphBB.width}px`)
  svgElement.setAttribute('height', `${graphBB.height}px`)

  const cache = new Map<string, DOMRect>()
  for (const line of lines.value) {
    if (!cache.has(line.p1)) {
      const p1 = document.getElementById(line.p1)
      if (!p1) {
        continue
      }
      cache.set(line.p1, p1.getBoundingClientRect())
    }
    if (!cache.has(line.p2)) {
      const p2 = document.getElementById(line.p2)
      if (!p2) {
        continue
      }
      cache.set(line.p2, p2.getBoundingClientRect())
    }
  }

  for (const line of lines.value) {
    const p1 = cache.get(line.p1)
    const p2 = cache.get(line.p2)
    if (!p1 || !p2) {
      continue
    }
    line.x1 = p1.x - svgBB.left + p1.width
    line.y1 = p1.y - svgBB.top + p1.height / 2
    line.x2 = p2.x - svgBB.left
    line.y2 = p2.y - svgBB.top + p2.height / 2
    line.angle =
      line.x2 == line.x1
        ? 0
        : (Math.atan((line.y2 - line.y1) / (line.x2 - line.x1)) * 180) / Math.PI
  }
}

const tick = () => {
  if (!mounted) {
    return
  }
  updateSize()
  window.requestAnimationFrame(tick)
}

let mounted = false
onMounted(() => {
  mounted = true
  tick()
})

onUnmounted(() => {
  mounted = false
})

const deletingTransport = ref(false)
const deletingDelivery = ref(false)
const deleteTransport = (transport: number) => {
  if (deletingTransport.value) {
    transports.value = transports.value.filter((t) => t !== transport)
    lines.value = lines.value.filter((line) => line.p2 !== `transport-${transport}`)
    lines.value = lines.value.filter((line) => line.p1 !== `transport-${transport}`)
    delete names.value[`transport-${transport}`]
    sortLines()
    deletingTransport.value = false
  }
}
const deleteDelivery = (delivery: number) => {
  if (deletingDelivery.value) {
    deliveries.value = deliveries.value.filter((d) => d !== delivery)
    lines.value = lines.value.filter((line) => line.p1 !== `delivery-${delivery}`)
    lines.value = lines.value.filter((line) => line.p2 !== `delivery-${delivery}`)
    delete names.value[`delivery-${delivery}`]
    sortLines()
    deletingDelivery.value = false
  }
}

const mapPoint = (point: string) => {
  if (point === 'startPoint') {
    return 1
  }
  if (point === 'endPoint') {
    return deliveries.value.length + transports.value.length + 2
  }
  if (point.startsWith('delivery-')) {
    return deliveries.value.indexOf(Number(point.substring(9))) + 2
  }
  if (point.startsWith('transport-')) {
    return transports.value.indexOf(Number(point.substring(10))) + deliveries.value.length + 2
  }
  throw new Error('Invalid point name')
}

const creatingPlan = ref(false)
const createPlan = async () => {
  deletingTransport.value = false
  deletingDelivery.value = false
  creatingPlan.value = true

  const edges: Edge[][] = []
  for (const line of lines.value) {
    if (line.maxFlow > 0) {
      const p1 = mapPoint(line.p1)
      const p2 = mapPoint(line.p2)
      while (edges.length < p1) {
        edges.push([])
      }
      edges[p1 - 1].push({
        to: p2,
        capacity: line.maxFlow,
        cost: line.cost
      })
    }
  }
  edges.push([])

  try {
    const result = await findMaxFlowMinCost(edges)
    for (const line of lines.value) {
      if (line.maxFlow > 0) {
        const p1 = mapPoint(line.p1)
        const p2 = mapPoint(line.p2)
        const edge = result[0][p1 - 1].find((e) => e.to === p2)
        if (edge) {
          line.flow = edge.used_capacity!
        }
      }
    }
  } finally {
    creatingPlan.value = false
  }
}

const hoveredLine = ref<Line | undefined>()
const mouseOver = (line: Line) => {
  hoveredLine.value = line
}
const mouseOut = (line: Line) => {
  if (hoveredLine.value === line) {
    hoveredLine.value = undefined
  }
}
</script>
<style scoped>
.flex-50 {
  flex-basis: 50%;
}

.min-h-100 {
  min-height: 100%;
}

.cursor-pointer {
  cursor: pointer;
}

input {
  width: 100%;
  --bs-border-color: rgba(0, 0, 0, 0);
  --bs-body-bg: none;
}

.shadow {
  filter: drop-shadow(0 0 0.75rem #000);
}

.hoverable {
  transition: opacity 0.3s;
}

.hovered-sth {
  opacity: 0.25;
}

.hovered-line {
  opacity: 1;
}
</style>
<template>
  <div class="d-flex flex-column h-100 w-100">
    <div class="w-100 overflow-auto position-relative flex-50">
      <b-row class="w-100 min-h-100" gutter-x="0" ref="graph">
        <b-col cols="3">
          <b-card
            header="Tartak"
            class="h-100 border-danger mx-3"
            header-class="text-center text-bg-danger"
            body-class="h4 d-flex flex-column align-items-center justify-content-around"
          >
            <b-badge variant="danger" class="text-wrap" pill="true" id="startPoint"
              >{{ names['startPoint'] }} ({{ mapPoint(`startPoint`) }})
            </b-badge>
          </b-card>
        </b-col>
        <b-col cols="3">
          <b-card
            header="Dostawcy"
            class="h-100 border-success mx-3"
            header-class="text-center text-bg-success"
            body-class="h4 d-flex flex-column align-items-center justify-content-around"
            footer-class="border-success text-center"
          >
            <template v-for="delivery of deliveries" :key="delivery.num">
              <b-badge
                variant="success"
                class="text-wrap"
                :class="{ 'cursor-pointer': deletingDelivery }"
                pill="true"
                :id="`delivery-${delivery}`"
                @click="deleteDelivery(delivery)"
                >{{ names[`delivery-${delivery}`] }} ({{ mapPoint(`delivery-${delivery}`) }})
              </b-badge>
            </template>
            <template #footer>
              <b-button
                class="mx-1"
                size="sm"
                variant="danger"
                v-if="deletingDelivery"
                @click="deletingDelivery = false"
                >Wybierz dostawcę
              </b-button>
              <b-button
                class="mx-1"
                size="sm"
                variant="danger"
                v-else
                @click="deletingDelivery = true"
                :disabled="creatingPlan"
                >Usuń
              </b-button>
              <b-button
                class="mx-1"
                size="sm"
                variant="success"
                @click="addDelivery"
                :disabled="creatingPlan"
                >Dodaj
              </b-button>
            </template>
          </b-card>
        </b-col>
        <b-col cols="3">
          <b-card
            header="Towary"
            class="h-100 border-info mx-3"
            header-class="text-center text-bg-info"
            body-class="h4 d-flex flex-column align-items-center justify-content-around"
            footer-class="border-info text-center"
          >
            <template v-for="transport of transports" :key="transport.num">
              <b-badge
                variant="info"
                class="text-wrap"
                :class="{ 'cursor-pointer': deletingTransport }"
                pill="true"
                :id="`transport-${transport}`"
                @click="deleteTransport(transport)"
                >{{ names[`transport-${transport}`] }} ({{ mapPoint(`transport-${transport}`) }})
              </b-badge>
            </template>
            <template #footer>
              <b-button
                class="mx-1"
                size="sm"
                variant="danger"
                v-if="deletingTransport"
                @click="deletingTransport = false"
                >Wybierz towar
              </b-button>
              <b-button
                class="mx-1"
                size="sm"
                variant="danger"
                v-else
                @click="deletingTransport = true"
                :disabled="creatingPlan"
                >Usuń
              </b-button>
              <b-button
                class="mx-1"
                size="sm"
                variant="info"
                @click="addTransport"
                :disabled="creatingPlan"
                >Dodaj
              </b-button>
            </template>
          </b-card>
        </b-col>
        <b-col cols="3">
          <b-card
            header="Fabryka"
            class="h-100 border-primary mx-3"
            header-class="text-center text-bg-primary"
            body-class="h4 d-flex flex-column align-items-center justify-content-around"
          >
            <b-badge variant="primary" class="text-wrap" pill="true" id="endPoint"
              >Fabryka ({{ mapPoint(`endPoint`) }})
            </b-badge>
          </b-card>
        </b-col>
      </b-row>
      <svg
        style="left: 0px; top: 0px"
        class="w-100 position-absolute"
        ref="svg"
        pointer-events="none"
      >
        <template v-for="line of lines" :key="line.id">
          <template v-if="line.maxFlow > 0">
            <g
              class="hoverable"
              :class="{
                'hovered-line': hoveredLine === line,
                'hovered-sth': hoveredLine !== undefined && hoveredLine.maxFlow > 0
              }"
            >
              <line
                :x1="line.x1"
                :y1="line.y1"
                :x2="line.x2"
                :y2="line.y2"
                :style="`stroke:${line.color};stroke-width:2`"
              />
              <polygon
                :points="`${line.x2},${line.y2} ${line.x2 - 10},${line.y2 - 10} ${line.x2 - 10},${line.y2 + 10}`"
                :style="`fill:${line.color}`"
                :transform="`rotate(${line.angle} ${line.x2} ${line.y2})`"
              />
              <text
                class="shadow"
                :x="(line.x1 + line.x2) / 2"
                :y="(line.y1 + line.y2) / 2"
                style="fill: white"
                dominant-baseline="ideographic"
                text-anchor="middle"
                :transform="`rotate(${line.angle} ${(line.x1 + line.x2) / 2} ${(line.y1 + line.y2) / 2})`"
              >
                {{ line.flow ?? '-' }} / {{ line.maxFlow }}
              </text>
              <text
                class="shadow"
                :x="(line.x1 + line.x2) / 2"
                :y="(line.y1 + line.y2) / 2"
                style="fill: white"
                dominant-baseline="hanging"
                text-anchor="middle"
                :transform="`rotate(${line.angle} ${(line.x1 + line.x2) / 2} ${(line.y1 + line.y2) / 2})`"
              >
                Koszt: {{ line.cost }}
              </text>
            </g>
          </template>
        </template>
      </svg>
    </div>
    <div class="overflow-auto flex-50">
      <b-table-simple small bordered hover>
        <b-thead>
          <b-tr class="text-center">
            <b-th>Skąd</b-th>
            <b-th>Dokąd</b-th>
            <b-th>Koszt</b-th>
            <b-th>Przepustowość</b-th>
            <b-th>Maksymalna przepustowość</b-th>
          </b-tr>
        </b-thead>
        <b-tbody>
          <b-tr
            v-for="line of lines"
            :key="line.id"
            @mouseover="mouseOver(line)"
            @mouseout="mouseOut(line)"
          >
            <b-td class="p-0">
              <b-form-input v-model="names[line.p1]" size="sm" />
            </b-td>
            <b-td class="p-0">
              <b-form-input v-model="names[line.p2]" size="sm" />
            </b-td>
            <b-td class="p-0">
              <b-form-input
                type="number"
                v-model.number="line.cost"
                size="sm"
                :disabled="creatingPlan"
              />
            </b-td>
            <b-td class="p-0 text-center">
              <b-progress
                v-if="line.flow"
                :value="line.flow"
                :max="line.maxFlow"
                :variant="line.flow === line.maxFlow ? 'success' : 'danger'"
                size="sm"
                show-value
                striped
                animated
              />
              <template v-else>-</template>
            </b-td>
            <b-td class="p-0">
              <b-form-input
                type="number"
                v-model.number="line.maxFlow"
                size="sm"
                :disabled="creatingPlan"
              />
            </b-td>
          </b-tr>
        </b-tbody>
        <b-tfoot>
          <b-tr>
            <b-th colspan="2" class="text-end px-3">
              Suma:
            </b-th>
            <b-th class="text-center">
              {{ lines.reduce((acc, line) => acc + line.cost * (line.flow ?? 0), 0) }}
            </b-th>
            <b-th class="text-center">
              {{ lines.filter((line) => line.p1 === 'startPoint').reduce((acc, line) => acc + (line.flow ?? 0), 0) }}
            </b-th>
            <b-td></b-td>
          </b-tr>
        </b-tfoot>
      </b-table-simple>
    </div>
    <b-button
      class="mx-auto"
      variant="primary"
      :disabled="deliveries.length === 0 || transports.length === 0 || creatingPlan"
      @click="createPlan"
      >Utwórz plan
    </b-button>
  </div>
</template>
