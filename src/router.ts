import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import ScheduleConflictView from '@/views/ScheduleConflictView.vue'
import DemoView from '@/views/DemoView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/conflict', component: ScheduleConflictView },
  { path: '/demo', component: DemoView }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
