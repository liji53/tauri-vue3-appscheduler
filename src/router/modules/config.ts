const Layout = () => import("@/layout/index.vue");

export default {
  path: "/config",
  redirect: "/config/config",
  component: Layout,
  meta: { title: "配置中心", icon: "flUser", rank: 9 },
  children: [
    {
      path: "/config/config",
      name: "Config",
      component: () => import("@/views/config/config.vue"),
      meta: { title: "配置管理" }
    }
  ]
} as RouteConfigsTable;
