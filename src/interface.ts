export interface BusinessTask {
  id: number
  name: string
  processTime: number
  weight: number
  conflicts: number[]
}
