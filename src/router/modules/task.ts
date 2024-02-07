const Layout = () => import("@/layout/index.vue");

export default {
  path: "/task",
  meta: { title: "任务中心", icon: "listCheck", rank: 2 },
  component: Layout,
  children: [
    {
      path: "/task/job",
      name: "Job",
      component: () => import("@/views/task/job.vue"),
      meta: { title: "任务管理" }
    }
  ]
} as RouteConfigsTable;
