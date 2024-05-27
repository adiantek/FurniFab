import { invoke } from '@tauri-apps/api'
import type { BusinessTask } from '@/composables/TaskComposable'

export interface CommandOutput {
  stdout: string
  stderr: string
  error: string[]
}

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

export function runExecutable(exec: string, stdin: string): Promise<CommandOutput> {
  return invoke('run_resource', { exec, stdin })
}

export async function scheduleConflicts(
  instance: Instance,
  algorithm: ConflictAlgorithm
): Promise<Schedule> {
  const scheduleString = await invoke('run_scheduling_conflicts', { instance, algorithm })
  return await JSON.parse(scheduleString as string)
}

export function scheduleFlow(tasks: FlowTask[], script: FlowScript): Promise<FlowSchedule> {
  return invoke('run_flow', { tasks, script })
}

export function exportApi(data: BusinessTask[]): Promise<void> {
  return invoke('export', { data: JSON.stringify(data) })
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

export interface Rect {
  id: number
  bin_id?: number
  x?: number
  y?: number
  w: number
  h: number
  color?: string
}
export function binPacking(bin: Bin, rects: Rect[], algorithm: BinPackingAlgorithm): Promise<Rect[]> {
  return invoke('run_bin_packing', { bin, rects, algorithm })
}

export async function importApi(): Promise<BusinessTask[]> {
  return JSON.parse(await invoke('import')).map(parseDates)
}

export function saveApi(data: BusinessTask[]): Promise<void> {
  return invoke('save_data', { data: JSON.stringify(data) })
}

export async function loadApi(): Promise<BusinessTask[]> {
  return JSON.parse(await invoke('load_data')).map(parseDates)
}
