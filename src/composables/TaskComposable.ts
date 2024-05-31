import { type Ref, ref, watch } from 'vue'
import { exportApi, importApi, loadApi, saveApi } from '@/api'

export interface BusinessTask {
  id: number
  name: string
  cuttingInfo: CuttingInfo
  flowInfo: FlowInfo,
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
}

const businessTasks = ref<BusinessTask[]>([])

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
  }  else {
    nextTaskId.value = 0
  }
}

export async function importData(): Promise<void> {
  const data = await importApi()
  loadData(data)
}

export async function exportData(): Promise<void> {
  await exportApi(businessTasks.value)
}

export function save(): Promise<void> {
  return saveApi(businessTasks.value)
}

export async function load(): Promise<void> {
  const data = await loadApi()
  loadData(data)
}
