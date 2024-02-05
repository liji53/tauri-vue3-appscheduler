import { invoke } from "@tauri-apps/api";

export type App = {
  name: string;
  url: string;
  category: string;
  description: string;
  status: string;
};
type AppResult = {
  total: number;
  data: Array<App>;
};

// app分类
export const getAppCategories = () => {
  return invoke<[string?]>("get_app_categories");
};

/** app列表 */
export const getAppList = (params: object) => {
  return invoke<AppResult>("get_apps", { ...params });
};

export const installApp = (app_name: string) => {
  return invoke("install_app", { app_name });
};

export const uninstallApp = (app_name: string) => {
  return invoke("uninstall_app", { app_name });
};

export const ungradeApp = (app_name: string) => {
  return invoke("ungrade_app", { app_name });
};

export const getAppReadme = (app_name: string) => {
  return invoke<string>("readme_app", { app_name });
};
