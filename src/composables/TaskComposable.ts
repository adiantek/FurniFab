import { type Ref, ref, watch } from 'vue'
import { exportApi, type ExportData, importApi, loadApi, saveApi } from '@/api'
import { useSuppliers, useSupplyPlan } from '@/composables/SupplierComposable'

export interface BusinessTask {
  id: number
  name: string
  cuttingInfo: CuttingInfo
  flowInfo: FlowInfo
  rectInfo: RectInfo
  materialInfo: MaterialInfo
}

export interface MaterialInfo {
  material: string
  amount: number
}

export interface CuttingInfo {
  conflicts: number[]
  processTime: number
  weight: number
  startingTime?: Date
  machine?: number
}

export interface FlowInfo {
  grindingProcessTime: number
  lacqueringProcessTime: number
  grinding?: [Date, number][]
  lacquering?: [Date, number][]
}

export interface RectInfo {
  id: number
  bin_id?: number
  x?: number
  y?: number
  w: number
  h: number
  color?: string
  mouseOver?: boolean
}

const businessTasks = ref<BusinessTask[]>([])

const boardHeight: Ref<number> = ref(100)
const boardWidth: Ref<number> = ref(100)

export function useBoardHeight(): Ref<number> {
  return boardHeight
}

export function useBoardWidth(): Ref<number> {
  return boardWidth
}

export const nextTaskId = ref<number>(0)

watch(businessTasks, save, { deep: true })

export function useBusinessTasks(): Ref<BusinessTask[]> {
  return businessTasks
}

export function getTask(id: number): BusinessTask | undefined {
  return businessTasks.value.find((task) => task.id === id)
}

export function getTaskIndex(list: BusinessTask[], id: number): number {
  return list.findIndex((task) => task.id === id)
}

function loadData(data: BusinessTask[] | null): void {
  if (data === null) {
    return
  }
  businessTasks.value = data
  if (data.length > 0) {
    nextTaskId.value = Math.max(...data.map((task) => task.id)) + 1
  } else {
    nextTaskId.value = 0
  }
}

export async function importData(): Promise<void> {
  const data = await importApi()
  loadData(data.businessTasks)
  useSuppliers().value = data.suppliers
  useSupplyPlan().value = data.supplyPlan
  useBoardWidth().value = data.boardSize[0]
  useBoardHeight().value = data.boardSize[1]
}

export async function exportData(): Promise<void> {
  const data: ExportData = {
    businessTasks: businessTasks.value,
    suppliers: useSuppliers().value,
    supplyPlan: useSupplyPlan().value,
    boardSize: [useBoardWidth().value, useBoardHeight().value]
  }
  await exportApi(data)
}

export function save(): Promise<void> {
  const data: ExportData = {
    businessTasks: businessTasks.value,
    suppliers: useSuppliers().value,
    supplyPlan: useSupplyPlan().value,
    boardSize: [useBoardWidth().value, useBoardHeight().value]
  }
  return saveApi(data)
}

export async function load(): Promise<void> {
  const data = await loadApi()

  if (data) {
    loadData(data.businessTasks)
    useSuppliers().value = data.suppliers
    useSupplyPlan().value = data.supplyPlan
    useBoardWidth().value = data.boardSize[0]
    useBoardHeight().value = data.boardSize[1]
  }
}
