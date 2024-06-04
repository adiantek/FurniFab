import { exportApi, importApi, loadApi, saveApi, type ExportData } from '@/api'
import { deliveryCounter, transportCounter, useSuppliers } from '@/composables/SupplierComposable'
import { ref, watch, type Ref } from 'vue'

export interface BusinessTask {
  id: number
  name: string
  cuttingInfo: CuttingInfo
  flowInfo: FlowInfo
  rectInfo: RectInfo
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

export const nextTaskId = ref(0)

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

function loadData(data: ExportData | null): void {
  if (data === null) {
    return
  }
  businessTasks.value = data.businessTasks
  if (data.businessTasks.length > 0) {
    nextTaskId.value = Math.max(...data.businessTasks.map((task) => task.id)) + 1
  } else {
    nextTaskId.value = 0
  }
  useBoardWidth().value = data.boardSize[0]
  useBoardHeight().value = data.boardSize[1]
  const { lines, deliveries, transports, names } = useSuppliers()
  if (data.lines !== undefined) {
    lines.value = data.lines
  }
  if (data.deliveries !== undefined) {
    deliveries.value = data.deliveries
    if (data.deliveries.length !== 0) {
      deliveryCounter.value = Math.max(...data.deliveries) + 1
    } else {
      deliveryCounter.value = 0
    }
  }
  if (data.transports !== undefined) {
    transports.value = data.transports
    if (data.transports.length !== 0) {
      transportCounter.value = Math.max(...data.transports) + 1
    } else {
      transportCounter.value = 0
    }
  }
  if (data.names !== undefined) {
    names.value = data.names
  }
}

export async function importData(): Promise<void> {
  const data = await importApi()
  loadData(data)
}

export async function exportData(): Promise<void> {
  const { lines, deliveries, transports, names } = useSuppliers()
  const data: ExportData = {
    businessTasks: businessTasks.value,
    boardSize: [useBoardWidth().value, useBoardHeight().value],
    lines: lines.value,
    deliveries: deliveries.value,
    transports: transports.value,
    names: names.value
  }
  await exportApi(data)
}

export function save(): Promise<void> {
  const { lines, deliveries, transports, names } = useSuppliers()
  const data: ExportData = {
    businessTasks: businessTasks.value,
    boardSize: [useBoardWidth().value, useBoardHeight().value],
    lines: lines.value,
    deliveries: deliveries.value,
    transports: transports.value,
    names: names.value
  }
  return saveApi(data)
}

export async function load(): Promise<void> {
  const data = await loadApi()

  if (data) {
    loadData(data)
  }
}
