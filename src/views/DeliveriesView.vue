<script setup lang="ts">
import { BinPackingAlgorithm, findMaxFlowMinCost } from '@/api'
import { type Supplier, useSuppliers, useSupplyPlan } from '@/composables/SupplierComposable'
import { useBusinessTasks } from '@/composables/TaskComposable'
import { ref } from 'vue'

const suppliers = useSuppliers()
const addCost = ref<[string, number, number]>(['', 0, 0])
const supplyPlan = useSupplyPlan()
const tasks = useBusinessTasks()

async function run() {
  const suppliersData = suppliers.value

  const mats = new Map<string, number>()

  tasks.value.forEach((task) => {
    if (!mats.has(task.materialInfo.material)) {
      mats.set(task.materialInfo.material, 0)
    }
    mats.set(
      task.materialInfo.material,
      mats.get(task.materialInfo.material)! + task.materialInfo.amount
    )
  })

  const materials = Array.from(new Set(tasks.value.map((task) => task.materialInfo.material)))

  const edges = [
    suppliersData.map((supp, i) => {
      return {
        to: i + 2,
        capacity: 4294967295,
        cost: 0
      }
    })
  ]

  suppliersData.forEach((supplier) => {
    edges.push(
      supplier.costs.map((delivery) => {
        return {
          to: materials.indexOf(delivery[0]) + 2 + suppliersData.length,
          capacity: delivery[1],
          cost: delivery[2]
        }
      })
    )
  })

  materials.forEach((material) => {
    edges.push([
      {
        to: materials.length + suppliersData.length + 2,
        capacity: mats.get(material)!,
        cost: 0
      }
    ])
  })

  edges.push([])

  const result = await findMaxFlowMinCost(edges)

  supplyPlan.value = [
    result[0]
      .filter((x, i) => {
        return i !== 0 && i! <= suppliersData.length
      })
      .map((costs, index) => {
        return {
          supplier: suppliersData[index].name,
          supplies: costs
            .filter((cost) => cost.to <= materials.length + suppliersData.length)
            .map((cost) => {
              return [materials[cost.to - 1 - suppliersData.length], cost.used_capacity!]
            })
        }
      }),
    result[1],
    result[2]
  ]
}

function removeSupplies(supplier: Supplier) {
  supplier.costs.push(addCost.value)
  addCost.value = ['', 0, 0]
}
</script>

<template>
  <b-row class="w-100 h-100">
    <b-col class="d-flex flex-column max-h-100 col-8">
      <h2>Dostawcy:</h2>
      <div class="overflow-x-auto flex-grow-1">
        <b-table-simple bordered small>
          <b-thead>
            <b-tr class="text-center">
              <b-th>Dostawca</b-th>
              <b-th>Maks dostawa</b-th>
              <b-th>Towar</b-th>
              <b-th>Maks</b-th>
              <b-th>Cena</b-th>
              <b-th>Akcje</b-th>
            </b-tr>
          </b-thead>
          <b-tbody>
            <template v-for="(supplier, i) of suppliers" :key="i">
              <b-tr v-for="(cost, index) of supplier.costs" :key="i + cost[1]">
                <b-td v-if="index === 0" class="text-center" :rowspan="supplier.costs.length + 1">
                  <button
                    class="btn btn-danger btn-sm m-1 float-end"
                    @click="() => (suppliers = suppliers.filter((_, i2) => i !== i2))"
                  >
                    <TrashIconComponent />
                  </button>
                  <div class="d-flex align-items-center">
                    <b-form-input v-model="supplier.name" placeholder="" />
                  </div>
                </b-td>

                <b-td v-if="index === 0" class="text-center" :rowspan="supplier.costs.length + 1">
                  <div class="d-flex align-items-center">
                    <b-form-input
                      type="number"
                      v-model.number="supplier.max_amount"
                      placeholder=""
                    />
                  </div>
                </b-td>

                <b-td>{{ cost[0] }}</b-td>
                <b-td>{{ cost[1] }}</b-td>
                <b-td>{{ cost[2] }}</b-td>
                <b-th>
                  <button
                    class="btn btn-danger btn-sm m-1 float-end"
                    @click="() => (supplier.costs = supplier.costs.filter((_, i) => i !== index))"
                  >
                    <TrashIconComponent />
                  </button>
                </b-th>
              </b-tr>
              <b-tr>
                <b-td
                  v-if="supplier.costs.length === 0"
                  class="text-center"
                  :rowspan="supplier.costs.length + 1"
                >
                  <button
                    class="btn btn-danger btn-sm m-1 float-end"
                    @click="() => (suppliers = suppliers.filter((_, i2) => i !== i2))"
                  >
                    <TrashIconComponent />
                  </button>
                  <div class="d-flex align-items-center">
                    <b-form-input v-model="supplier.name" placeholder="" />
                  </div>
                </b-td>

                <b-td
                  v-if="supplier.costs.length === 0"
                  class="text-center"
                  :rowspan="supplier.costs.length + 1"
                >
                  <div class="d-flex align-items-center">
                    <b-form-input
                      type="number"
                      v-model.number="supplier.max_amount"
                      placeholder=""
                    />
                  </div>
                </b-td>

                <b-td>
                  <div class="d-flex align-items-center">
                    <b-form-input v-model="addCost[0]" placeholder="Towar" />
                  </div>
                </b-td>
                <b-td>
                  <div class="d-flex align-items-center">
                    <b-form-input type="number" v-model.number="addCost[1]" placeholder="Maks" />
                  </div>
                </b-td>
                <b-td>
                  <div class="d-flex align-items-center">
                    <b-form-input type="number" v-model.number="addCost[2]" placeholder="Cena" />
                  </div>
                </b-td>
                <b-td>
                  <button class="btn btn-success m-1 float-end" @click="removeSupplies(supplier)">
                    +
                  </button>
                </b-td>
              </b-tr>
            </template>
          </b-tbody>
        </b-table-simple>
        <button
          class="btn btn-success m-1 float-end"
          @click="suppliers.push({ name: '', max_amount: 0, costs: [] })"
        >
          Dodaj dostawcę
        </button>
      </div>
      <LoadingButton
        class="mx-auto"
        :disabled="suppliers.length === 0 && tasks.length === 0"
        @click="run"
        >Utwórz plan dostaw
      </LoadingButton>
    </b-col>
    <b-col class="overflow-auto">
      <h2>Plan dostaw:</h2>
      <!--      <svg class="w-100 h-100" :viewBox="`0 0 ${binW} ${binH}`">-->
      <!--        <rect :width="binW" :height="binH" fill="#40404040" />-->
      <!--        <template v-for="task of businessTasks" :key="task.id">-->
      <!--          <rect v-if="task.rectInfo.x !== -1 && task.rectInfo.x !== undefined" :width="task.rectInfo.w"-->
      <!--                :height="task.rectInfo.h" :x="task.rectInfo.x" :y="task.rectInfo.y" :fill="task.rectInfo.color" />-->
      <!--        </template>-->
      <!--      </svg>-->
    </b-col>
  </b-row>
</template>
