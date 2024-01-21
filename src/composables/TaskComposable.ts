import { type Ref, ref } from 'vue'

export interface BusinessTask {
  id: number
  name: string
  cuttingInfo: CuttingInfo
  flowInfo: FlowInfo
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

const businessTasks = ref<BusinessTask[]>([
  {
    id: 0,
    name: 'Zadanie 0',
    cuttingInfo: { processTime: 150, weight: 8, conflicts: [5, 1] },
    flowInfo: { grindingProcessTime: 90, lacqueringProcessTime: 60 }
  },
  {
    id: 1,
    name: 'Zadanie 1',
    cuttingInfo: { processTime: 120, weight: 1, conflicts: [0, 6, 7] },
    flowInfo: { grindingProcessTime: 60, lacqueringProcessTime: 80 }
  },
  {
    id: 2,
    name: 'Zadanie 2',
    cuttingInfo: { processTime: 140, weight: 4, conflicts: [3, 4, 7] },
    flowInfo: { grindingProcessTime: 80, lacqueringProcessTime: 60 }
  },
  {
    id: 3,
    name: 'Zadanie 3',
    cuttingInfo: { processTime: 80, weight: 3, conflicts: [2, 7] },
    flowInfo: { grindingProcessTime: 40, lacqueringProcessTime: 40 }
  },
  {
    id: 4,
    name: 'Zadanie 4',
    cuttingInfo: { processTime: 130, weight: 7, conflicts: [2, 5, 9] },
    flowInfo: { grindingProcessTime: 70, lacqueringProcessTime: 60 }
  },
  {
    id: 5,
    name: 'Zadanie 5',
    cuttingInfo: { processTime: 200, weight: 11, conflicts: [0, 4, 9] },
    flowInfo: { grindingProcessTime: 100, lacqueringProcessTime: 100 }
  },
  {
    id: 6,
    name: 'Zadanie 6',
    cuttingInfo: { processTime: 60, weight: 2, conflicts: [1] },
    flowInfo: { grindingProcessTime: 40, lacqueringProcessTime: 20 }
  },
  {
    id: 7,
    name: 'Zadanie 7',
    cuttingInfo: { processTime: 240, weight: 5, conflicts: [1, 2, 3] },
    flowInfo: { grindingProcessTime: 120, lacqueringProcessTime: 120 }
  },
  {
    id: 8,
    name: 'Zadanie 8',
    cuttingInfo: { processTime: 80, weight: 1, conflicts: [9] },
    flowInfo: { grindingProcessTime: 40, lacqueringProcessTime: 40 }
  },
  {
    id: 9,
    name: 'Zadanie 9',
    cuttingInfo: { processTime: 150, weight: 3, conflicts: [4, 5, 8] },
    flowInfo: { grindingProcessTime: 90, lacqueringProcessTime: 60 }
  }
])

export const nextTaskId = ref<number>(10)

export function useBusinessTasks(): Ref<BusinessTask[]> {
  return businessTasks
}

export function getTask(id: number): BusinessTask | undefined {
  return businessTasks.value.find((task) => task.id === id)
}

export function getTaskIndex(list: BusinessTask[], id: number): number {
  return list.findIndex((task) => task.id === id)
}
