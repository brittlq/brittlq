import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () =>
      import(/* webpackChunkName: "queue" */ '@/views/Queue.vue'),
  },
  {
    path: '/party-queue',
    name: 'Queue',
    component: () =>
      import(/* webpackChunkName: "queue" */ '../views/Queue.vue'),
  },
  {
    path: '/about',
    name: 'About',
    component: () =>
      import(/* webpackChunkName: "about" */ '../views/About.vue'),
  },
  {
    path: '/commands',
    name: 'Commands',
    component: () =>
      import(/* webpackChunkName: "commands" */ '../views/Commands.vue'),
  },
  {
    path: '/obs',
    name: 'OBS Controls',
    component: () => import(/* webpackChunkName: "obs" */ '../views/OBS.vue'),
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () =>
      import(/* webpackChunkName: "settings" */ '@/views/Settings.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
