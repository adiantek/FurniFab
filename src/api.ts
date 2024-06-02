import { invoke } from '@tauri-apps/api'
import type { BusinessTask, RectInfo } from '@/composables/TaskComposable'
import { useToast } from 'bootstrap-vue-next'
import type { Supplier, SupplyPlan } from '@/composables/SupplierComposable'

export enum ConflictAlgorithm {
  List = 'List',
  VNS = 'VNS',
  Tresoldi = 'Tresoldi'
}

export interface ConflictTask {
  processing_time: number
  weight: number
}

export type Conflict = [number, number]

export type ConflictGraph = Conflict[]

export interface Instance {
  processors: number
  deadline: number
  tasks: ConflictTask[]
  graph: ConflictGraph
}

export interface ScheduleInfo {
  processor: number
  start_time: number
}

export interface Schedule {
  schedule: (ScheduleInfo | null)[]
}

export enum FlowScript {
  Pa = 'Pa',
  Johnson = 'Johnson',
  Johnson2 = 'Johnson2',
  BranchAndBound = 'BranchAndBound',
  Neh = 'Neh'
}

export interface FlowTask {
  start_time: number
  grinding_time: number
  lacquering_time: number
}

export interface FlowScheduleInfo {
  start_time: number
  end_time: number
}

export interface FlowSchedule {
  grinding: FlowScheduleInfo[][]
  lacquering: FlowScheduleInfo[][]
}

export async function scheduleConflicts(
  instance: Instance,
  algorithm: ConflictAlgorithm
): Promise<Schedule> {
  const scheduleString = await invoke('run_scheduling_conflicts', { instance, algorithm }).catch(
    onError
  )
  return await JSON.parse(scheduleString as string)
}

export function scheduleFlow(tasks: FlowTask[], script: FlowScript): Promise<FlowSchedule> {
  return invoke('run_flow', { tasks, script }).catch(onError) as Promise<FlowSchedule>
}

export interface Edge {
  to: number
  capacity: number
  used_capacity?: number
  cost: number
}

export function findMaxFlowMinCost(edges: Edge[][]): Promise<[Edge[][], number, number]> {
  return invoke('run_max_flow_min_cost', { edges }).catch(onError) as Promise<[Edge[][], number, number]>
}

export interface ExportData {
  businessTasks: BusinessTask[]
  suppliers: Supplier[]
  supplyPlan?: SupplyPlan
  boardSize: [number, number]
}

export function exportApi(data: ExportData): Promise<void> {
  return invoke('export', { data: JSON.stringify(data) }).catch(onError) as Promise<void>
}

function parseDates(task: BusinessTask): BusinessTask {
  if (task.cuttingInfo.startingTime) {
    task.cuttingInfo.startingTime = new Date(task.cuttingInfo.startingTime)
  }
  const flow = task.flowInfo
  if (flow.grinding && flow.lacquering) {
    flow.grinding = flow.grinding.map((value) => [new Date(value[0]), value[1]])
    flow.lacquering = flow.lacquering.map((value) => [new Date(value[0]), value[1]])
  }
  return task
}

export enum BinPackingAlgorithm {
  FFDH = 'FFDH'
}

export interface Bin {
  id: number
  w: number
  h: number
}

export function binPacking(
  bin: Bin,
  rects: RectInfo[],
  algorithm: BinPackingAlgorithm
): Promise<RectInfo[]> {
  return invoke('run_bin_packing', { bin, rects, algorithm })
}

export async function importApi(): Promise<ExportData> {
  const data: ExportData | undefined = JSON.parse((await invoke('import').catch(onError)) as string)

  if (data) {
    data.businessTasks = data.businessTasks.map(parseDates)
  }

  return data || { businessTasks: [], suppliers: [], boardSize: [100, 100] }
}

export function saveApi(data: ExportData): Promise<void> {
  return invoke('save_data', { data: JSON.stringify(data) }).catch(onError) as Promise<void>
}

export async function loadApi(): Promise<ExportData> {
  const data: ExportData = JSON.parse((await invoke('load_data').catch(onError)) as string);
  data.businessTasks = data.businessTasks.map(parseDates)
  return data
}

function onError(error: any) {
  const { show } = useToast()
  show?.(error.toString(), {
    title: 'Błąd w przetwarzaniu',
    value: 5000,
    variant: 'danger',
    pos: 'bottom-right'
  })
}
