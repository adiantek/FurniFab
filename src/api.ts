import { invoke } from '@tauri-apps/api'

export interface CommandOutput {
  stdout: string
  stderr: string
  error: string[]
}

export enum ConflictAlgorithm {
  List = 'List',
  VNS = 'VNS'
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
  BranchAndBound = 'BranchAndBound'
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
