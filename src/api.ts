import { invoke } from '@tauri-apps/api'

export interface CommandOutput {
  stdout: string
  stderr: string
  error: string[]
}

export function runExecutable(exec: string, stdin: string): Promise<CommandOutput> {
  return invoke('run_resource', { exec, stdin })
}
