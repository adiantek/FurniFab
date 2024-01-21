import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import ScheduleConflictView from '@/views/ScheduleConflictView.vue'
import DemoView from '@/views/DemoView.vue'
import TaskListView from '@/views/TaskListView.vue'
import ScheduleFlowView from '@/views/ScheduleFlowView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/tasks', component: TaskListView },
  { path: '/conflict', component: ScheduleConflictView },
  { path: '/flow', component: ScheduleFlowView },
  { path: '/demo', component: DemoView }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
