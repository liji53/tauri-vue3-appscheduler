import { invoke } from "@tauri-apps/api";

type Job = {
  id: number;
  name: string;
  remark: string;
  status: boolean; // 是否启用
  next_at: string;
  // updated_at: string;
  cron: string;

  app_name: string;
  category: string;
  url: string;
};
type JobResult = {
  total: number;
  data: Array<Job>;
};

export type Log = {
  created_at: string; // 执行时间
  content: string;
};

type TaskResult = {
  created_at: string; // 执行时间
  html_path: string;
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

// 任务执行日志
export const getJobLog = (job_id: number) => {
  return invoke<Log>("get_job_log", { id: job_id });
};

// 获取任务执行结果
export const getJobResult = (job_id: number) => {
  return invoke<TaskResult>("get_job_result", { id: job_id });
};
