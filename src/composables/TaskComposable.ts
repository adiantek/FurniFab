import { type Ref, ref } from 'vue'

export interface BusinessTask {
  id: number
  name: string
  cuttingInfo: CuttingInfo
  flowInfo?: FlowInfo
}

export interface CuttingInfo {
  conflicts: number[]
  processTime: number
  weight: number
  startingTime?: number
  machine?: number
}

export interface FlowInfo {
  firstMachine: [[number, number]]
  secondMachine: [[number, number]]
}

const businessTasks = ref<BusinessTask[]>([
  { id: 0, name: 'Zadanie 0', cuttingInfo: { processTime: 75, weight: 8, conflicts: [5, 1] } },
  { id: 1, name: 'Zadanie 1', cuttingInfo: { processTime: 60, weight: 1, conflicts: [0, 6, 7] } },
  { id: 2, name: 'Zadanie 2', cuttingInfo: { processTime: 70, weight: 4, conflicts: [3, 4, 7] } },
  { id: 3, name: 'Zadanie 3', cuttingInfo: { processTime: 40, weight: 3, conflicts: [2, 7] } },
  { id: 4, name: 'Zadanie 4', cuttingInfo: { processTime: 65, weight: 7, conflicts: [2, 5, 9] } },
  { id: 5, name: 'Zadanie 5', cuttingInfo: { processTime: 100, weight: 11, conflicts: [0, 4, 9] } },
  { id: 6, name: 'Zadanie 6', cuttingInfo: { processTime: 30, weight: 2, conflicts: [1] } },
  { id: 7, name: 'Zadanie 7', cuttingInfo: { processTime: 120, weight: 5, conflicts: [1, 2, 3] } },
  { id: 8, name: 'Zadanie 8', cuttingInfo: { processTime: 40, weight: 1, conflicts: [9] } },
  { id: 9, name: 'Zadanie 9', cuttingInfo: { processTime: 75, weight: 3, conflicts: [4, 5, 8] } }
])

export const nextTaskId = ref<number>(10)

export function useBusinessTasks(): Ref<BusinessTask[]> {
  return businessTasks
}

export function getTask(id: number): BusinessTask | undefined {
  return businessTasks.value.find((task) => task.id === id)
}
