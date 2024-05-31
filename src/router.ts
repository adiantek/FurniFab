import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import ScheduleConflictView from '@/views/ScheduleConflictView.vue'
import BinPacking from '@/views/BinPacking.vue'
import TaskListView from '@/views/TaskListView.vue'
import ScheduleFlowView from '@/views/ScheduleFlowView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/tasks', component: TaskListView },
  { path: '/conflict', component: ScheduleConflictView },
  { path: '/flow', component: ScheduleFlowView },
  { path: '/binpacking', component: BinPacking }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
