import { reactive, ref, onMounted } from "vue";
import {
  type App,
  getAppList,
  getAppReadme,
  getAppCategories,
  uninstallApp,
  installApp,
  ungradeApp
} from "@/api/app";
import { message } from "@/utils/message";

export function useApp() {
  const loading = ref(true);
  const pagination = reactive({
    total: 0,
    pageSize: 8,
    currentPage: 1
  });

  // 搜索表单
  const form = reactive({
    appName: "",
    category: "",
    status: "",
    page: 1,
    itemsPerPage: 8
  });

  // 后台数据
  const categories = ref<[string?]>([]);
  const apps = ref<Array<App>>([]);
  const readme = ref("");

  // 分页
  const onPageSizeChange = (val: number) => {
    form.page = 1;
    form.itemsPerPage = val;
    onSearch();
  };
  const onCurrentPageChange = (val: number) => {
    form.page = val;
    onSearch();
  };

  // 搜索表单的响应
  function onSearch() {
    loading.value = true;
    getAppList(form)
      .then(response => {
        apps.value = response.data;
        pagination.total = response.total;
      })
      .catch((err: string) => {
        message(err, { type: "error" });
      })
      .finally(() => {
        loading.value = false;
      });
  }
  const resetForm = formEl => {
    if (!formEl) return;
    formEl.resetFields();
    onSearch();
  };

  // 用于响应AppCard的事件
  const handleInstallApp = (app: App) => {
    loading.value = true;
    installApp(app.url)
      .then(() => {
        message("应用安装成功!", { type: "success" });
        onSearch();
      })
      .catch((err: string) => {
        message(err, { type: "error" });
      })
      .finally(() => {
        loading.value = false;
      });
  };
  const handleUninstallApp = (app: App) => {
    uninstallApp(app.url)
      .then(() => {
        message("应用卸载成功!", { type: "success" });
        onSearch();
      })
      .catch((err: string) => {
        message(err, { type: "error" });
      });
  };
  const handleUpgradeApp = (app: App) => {
    loading.value = true;
    ungradeApp(app.url)
      .then(() => {
        message("应用升级成功!", { type: "success" });
        onSearch();
      })
      .catch((err: string) => {
        message(err, { type: "error" });
      })
      .finally(() => {
        loading.value = false;
      });
  };
  const handleReadmeApp = (app: App) => {
    getAppReadme(app.url)
      .then((response: string) => {
        readme.value = response;
      })
      .catch((err: string) => {
        message(err, { type: "error" });
        readme.value = "";
      });
  };

  onMounted(() => {
    onSearch();
    getAppCategories()
      .then(response => {
        categories.value = response;
      })
      .catch((err: string) => {
        message(err, { type: "error" });
      });
  });

  return {
    form,
    loading,
    pagination,
    categories,
    apps,
    readme,
    onSearch,
    resetForm,
    onPageSizeChange,
    onCurrentPageChange,
    handleInstallApp,
    handleReadmeApp,
    handleUninstallApp,
    handleUpgradeApp
  };
}
