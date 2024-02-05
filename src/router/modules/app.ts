const Layout = () => import("@/layout/index.vue");

export default {
  path: "/app",
  redirect: "/app/store",
  component: Layout,
  meta: { title: "应用中心", icon: "menu", rank: 5 },
  children: [
    {
      path: "/app/store",
      name: "Store",
      component: () => import("@/views/app/store.vue"),
      meta: {
        title: "应用商城"
      }
    }
  ]
} as RouteConfigsTable;
