import { createRouter, createWebHistory } from "vue-router";
import HomeView from '../views/HomeView.vue'
import WelcomeAuth from '../views/auth/WelcomeAuth.vue'
import LoginView from '../views/auth/LoginView.vue'
import RegisterView from '../views/auth/RegisterView.vue'
import SplashView from '../views/SplashView.vue'
import { useUserStore } from '../stores/user'

const routes = [
  {
    path: "/",
    name: "splash",
    component: SplashView
  },
  {
    path: "/welcome",
    name: "welcome",
    component: WelcomeAuth
  },
  {
    path: "/login",
    name: "login",
    component: LoginView
  },
  {
    path: "/register",
    name: "register",
    component: RegisterView
  },
      {
        path: "/home",
        name: "home",
    component: HomeView,
    meta: { requiresAuth: true }
      },
      {
        path: "/protect",
        meta: { requiresAuth: true },
        name: "protect",
        component: () => import("../views/ProtectView.vue"),
      },
      {
        path: "/list",
        meta: { requiresAuth: true },
        name: "list",
        component: () => import("../views/ListView.vue"),
      },
      {
        path: "/async",
        meta: { requiresAuth: true },
        name: "async",
        component: () => import("../views/AsyncView.vue"),
      },
      {
        path: "/async-upload",
        meta: { requiresAuth: true },
        name: "async-upload",
        component: () => import("../views/AsyncUpload.vue"),
      },
      {
        path: "/config",
        name: "config",
        component: () => import("../views/ConfigView.vue"),
  },
  {
    path: "/404",
    name: "404",
    component: () => import("../views/NotFound.vue"),
  },
  // 所有未定义路由，全部重定向到 404 页
  { path: "/:pathMatch(.*)*", redirect: "/404" },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

// 路由守卫
router.beforeEach((to, from, next) => {
  const userStore = useUserStore()
  
  // 启动页面不需要认证检查
  if (to.name === 'splash') {
    next()
    return
  }
  
  // 如果需要登录且用户未认证
  if (to.meta.requiresAuth && !userStore.isAuthenticated) {
    next('/login')
  } else {
    next()
  }
})

export default router;
