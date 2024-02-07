import { invoke } from "@tauri-apps/api";

type Job = {
  id: number;
  name: string;
  remark: string;
  status: boolean; // 是否启用
  next_at: string;
  updated_at: string;
  cron: string;

  app_name: string;
  category: string;
  url: string;
};
type JobResult = {
  total: number;
  data: Array<Job>;
};

// 获取任务列表
export const getJobList = (params: object) => {
  return invoke<JobResult>("get_jobs", { ...params });
};

export const createJob = (data: object) => {
  return invoke("create_job", { data: JSON.stringify(data) });
};

export const updateJob = (job_id: number, data: object) => {
  return invoke("update_job", { id: job_id, data: JSON.stringify(data) });
};

export const deleteJob = (job_id: number) => {
  return invoke("delete_job", { id: job_id });
};

// 手动执行任务
export const runJob = (job_id: number) => {
  return invoke("run_job", { id: job_id });
};

// 获取任务的配置
export const getJobConfig = (job_id: number) => {
  return invoke("get_job_config", { id: job_id });
};

// 设置任务的配置
export const setJobConfig = (job_id: number, data: object) => {
  return invoke("set_job_config", { id: job_id, data: JSON.stringify(data) });
};
