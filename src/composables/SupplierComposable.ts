import { type Ref, ref, watch } from 'vue'
import { save } from '@/composables/TaskComposable'

export interface Supplier {
  name: string
  max_amount: number
  costs: [string, number, number][]
}

const suppliers = ref<Supplier[]>([])

watch(suppliers, save, { deep: true })

export function useSuppliers(): Ref<Supplier[]> {
  return suppliers
}

export type SupplyPlan = [{ supplier: string; supplies: [string, number][] }[], number, number]

const supplyPlan = ref<SupplyPlan | undefined>()

watch(supplyPlan, save, { deep: true })

export function useSupplyPlan(): Ref<SupplyPlan | undefined> {
  return supplyPlan
}

export function setSupplyPlan(plan: SupplyPlan): void {
  supplyPlan.value = plan
}
