interface JobItemProps {
  // 主键
  id?: number;
  /** 任务名称 */
  name: string;
  /** 备注 */
  remark: string;
  /* 安装的应用名 */
  app_name: string;
  apps: Array<any>;
}
interface JobFormProps {
  formInline: JobItemProps;
}

export type { JobFormProps, JobItemProps };
