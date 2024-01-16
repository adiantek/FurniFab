import { invoke } from '@tauri-apps/api'

export interface CommandOutput {
  stdout: string
  stderr: string
  error: string[]
}

export interface Task {
  processing_time: number
  weight: number
}

export type Conflict = [number, number]

export type ConflictGraph = Conflict[]

export interface Instance {
  processors: number
  deadline: number
  tasks: Task[]
  graph: ConflictGraph
}

export interface ScheduleInfo {
  processor: number
  start_time: number
}

export interface Schedule {
  schedule: (ScheduleInfo | null)[]
}

export type ScheduleWithScore = [Schedule, number]

export function runExecutable(exec: string, stdin: string): Promise<CommandOutput> {
  return invoke('run_resource', { exec, stdin })
}

export async function scheduleWithConflicts(instance: Instance): Promise<ScheduleWithScore> {
  const scheduleString = await invoke('run_scheduling_conflicts', { instance })
  return await JSON.parse(scheduleString as string)
}
