import { type Ref, ref } from 'vue'

export interface Supplier {
  name: string
  max_amount: number
  costs: [string, number, number][]
}

const suppliers = ref<Supplier[]>([])

export function useSuppliers(): Ref<Supplier[]> {
  return suppliers
}

export type SupplyPlan = [{ supplier: string; supplies: [string, number][] }[], number, number]

const supplyPlan = ref<SupplyPlan | undefined>()

export function useSupplyPlan(): Ref<SupplyPlan | undefined> {
  return supplyPlan
}

export function setSupplyPlan(plan: SupplyPlan): void {
  supplyPlan.value = plan
}
