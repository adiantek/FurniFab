import { createRouter, createWebHashHistory } from 'vue-router'
import ScheduleConflictView from '@/views/ScheduleConflictView.vue'
import BinPackingView from '@/views/BinPackingView.vue'
import DeliveriesView from '@/views/DeliveriesView.vue'
import TaskListView from '@/views/TaskListView.vue'
import ScheduleFlowView from '@/views/ScheduleFlowView.vue'

const routes = [
  { path: '/', component: TaskListView },
  { path: '/conflict', component: ScheduleConflictView },
  { path: '/flow', component: ScheduleFlowView },
  { path: '/binpacking', component: BinPackingView },
  { path: '/deliveries', component: DeliveriesView }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
