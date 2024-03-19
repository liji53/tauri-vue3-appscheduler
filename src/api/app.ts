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

// 应用的tree形式
type AppTree = {
  name: string;
  children?: Array<AppTree>;
};
export type AppTreeResult = Array<AppTree>;

// app分类
export const getAppCategories = () => {
  return invoke<[string?]>("get_app_categories");
};

/** app列表 */
export const getAppList = (params: object) => {
  return invoke<AppResult>("get_apps", { ...params });
};

export const installApp = (repoUrl: string, isInstallByVenv: boolean) => {
  return invoke("install_app", { repoUrl, isInstallByVenv });
};

export const uninstallApp = (repoUrl: string) => {
  return invoke("uninstall_app", { repoUrl });
};

export const ungradeApp = (repoUrl: string) => {
  return invoke("ungrade_app", { repoUrl });
};

export const getAppReadme = (repoUrl: string) => {
  return invoke<string>("readme_app", { repoUrl });
};

export const getAppTree = () => {
  return invoke<AppTreeResult>("get_app_tree");
};
