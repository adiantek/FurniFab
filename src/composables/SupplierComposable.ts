import { save } from '@/composables/TaskComposable'
import type { Line } from '@/views/DeliveriesView.vue'
import { ref, watch } from 'vue'

const lines = ref<Line[]>([])
const deliveries = ref<number[]>([])
const transports = ref<number[]>([])
const names = ref<Record<string, string>>({
  startPoint: 'Tartak',
  endPoint: 'Fabryka'
})
export const deliveryCounter = ref(0)
export const transportCounter = ref(0) 

export const useSuppliers = () => {
  return {
    lines,
    deliveries,
    transports,
    names
  }
}

watch(lines, save, { deep: true })
watch(deliveries, save, { deep: true })
watch(transports, save, { deep: true })
watch(names, save, { deep: true })
