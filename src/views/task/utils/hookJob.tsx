import dayjs from "dayjs";
import editForm from "../components/jobForm.vue";
import { message } from "@/utils/message";
import {
  getJobList,
  createJob,
  updateJob,
  deleteJob,
  runJob,
  getJobConfig,
  setJobConfig,
  type Log,
  getJobLog,
  getJobResult
} from "@/api/job";
import { getAppTree, type AppTreeResult } from "@/api/app";
import { ElMessageBox } from "element-plus";
import { addDialog } from "@/components/ReDialog";
import { type JobItemProps } from "./types";
import { type PaginationProps } from "@pureadmin/table";
import { reactive, ref, onMounted, h, computed, toRaw } from "vue";
import { open } from "@tauri-apps/api/shell";

const nextAtStyle = computed(() => {
  return (next_at: string) => {
    return next_at == null
      ? {
          "--el-tag-text-color": "#cf1322",
          "--el-tag-bg-color": "#fff1f0",
          "--el-tag-border-color": "#ffa39e"
        }
      : {};
  };
});

export function useJob() {
  // header 表单的数据
  const form = reactive({
    name: "",
    category: "",
    page: 1,
    itemsPerPage: 10
  });
  const dataList = ref([]); // 任务列表
  const apps = ref<AppTreeResult>(); // 安装的应用的树结构
  const categories = ref([]);
  const loading = ref(true);
  const formRef = ref();
  const switchLoadMap = ref({});
  const switchStyle = computed(() => {
    return {
      "--el-switch-on-color": "#6abe39",
      "--el-switch-off-color": "#e84749"
    };
  });
  const pagination = reactive<PaginationProps>({
    total: 0,
    pageSize: 10,
    currentPage: 1
  });
  // 定时设置
  const crontabVisible = ref(false);
  const cronFormData = reactive({
    id: null,
    cron: "* * * * *"
  });
  // 任务配置
  const taskConifgVisible = ref(false);
  const taskRow = ref(); // 设置配置时记录当前任务id
  const taskConfigData = ref([]); // 任务的当前配置
  // 运行日志
  const logVisible = ref(false);
  const log = ref<Log>({
    created_at: "",
    content: ""
  });

  const columns: TableColumnList = [
    {
      label: "任务名称",
      prop: "name",
      minWidth: 120
    },
    {
      label: "应用分类",
      prop: "category",
      minWidth: 120
    },
    {
      label: "状态",
      minWidth: 80,
      cellRenderer: scope => (
        <el-switch
          size={scope.props.size === "small" ? "small" : "default"}
          loading={switchLoadMap.value[scope.index]?.loading}
          v-model={scope.row.status}
          active-value={true}
          inactive-value={false}
          active-text="启用"
          inactive-text="停用"
          inline-prompt
          style={switchStyle.value}
          onChange={() => onChangeStatus(scope as any)}
        />
      )
    },
    {
      label: "备注",
      prop: "remark",
      minWidth: 150
    },
    {
      label: "下次执行时间",
      minWidth: 160,
      prop: "next_at",
      cellRenderer: ({ row }) => (
        <el-tag size="small" style={nextAtStyle.value(row.next_at)}>
          {row.next_at != null
            ? dayjs(row.next_at).format("YYYY-MM-DD HH:mm:ss")
            : "未上线"}
        </el-tag>
      )
    },
    // {
    //   label: "更新时间",
    //   minWidth: 160,
    //   prop: "updated_at",
    //   formatter: ({ updated_at }) =>
    //     dayjs(updated_at).format("YYYY-MM-DD HH:mm:ss")
    // },
    {
      label: "操作",
      fixed: "right",
      width: 270,
      slot: "operation"
    }
  ];

  // 使能状态切换
  function onChangeStatus({ row, index }) {
    ElMessageBox.confirm(
      `确认要<strong>${
        row.status === false ? "停用" : "启用"
      }</strong><strong style='color:var(--el-color-primary)'>${
        row.name
      }</strong>吗?`,
      "系统提示",
      {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        type: "warning",
        dangerouslyUseHTMLString: true,
        draggable: true
      }
    )
      .then(() => {
        switchLoadMap.value[index] = Object.assign(
          {},
          switchLoadMap.value[index],
          {
            loading: true
          }
        );
        updateJob(row.id, { status: row.status })
          .then(() => {
            message(
              `已${row.status === false ? "停用" : "启用"}【${row.name}】`,
              {
                type: "success"
              }
            );
          })
          .catch(() => {
            // 请求失败，恢复状态
            row.status === false ? (row.status = true) : (row.status = false);
          });
        // 加载状态恢复
        switchLoadMap.value[index] = Object.assign(
          {},
          switchLoadMap.value[index],
          {
            loading: false
          }
        );
      })
      .catch(() => {
        // 取消 恢复状态
        row.status === false ? (row.status = true) : (row.status = false);
      });
  }
  // 手动执行任务(异步，在通知栏中listen任务完成的事件)
  function handleRun(row) {
    runJob(row.id)
      .then(() => {
        // 异步执行
        message(`开始运行【${row.name}】`, {
          type: "success"
        });
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  }

  // 定时任务相关逻辑
  function handleTimer(row) {
    cronFormData.id = row["id"];
    cronFormData.cron = row["cron"];
    crontabVisible.value = true;
  }
  const onCancelCron = () => {
    crontabVisible.value = false;
  };
  const onConfirmCron = async () => {
    updateJob(cronFormData.id, { cron: cronFormData.cron })
      .then(() => {
        message(`设置定时任务成功`, {
          type: "success"
        });
        crontabVisible.value = false;
        onSearch();
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  };

  // 查看当前任务的配置
  function handleConfig(row: JobItemProps) {
    getJobConfig(row.id)
      .then((response: string) => {
        taskConfigData.value = JSON.parse(response);
        taskConifgVisible.value = true;
        taskRow.value = row;
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  }
  // 保存当前任务的配置
  function handleConfirmConfig(formRef, config) {
    formRef.validate(async valid => {
      if (valid) {
        setJobConfig(taskRow.value.id, config)
          .then(() => {
            message("配置设置成功", { type: "success" });
          })
          .catch((error: string) => {
            message(error, { type: "error" });
          });
      }
    });
  }

  // 删除任务
  function handleDelete(row) {
    deleteJob(row.id)
      .then(() => {
        message(`删除任务成功`, { type: "success" });
        onSearch();
      })
      .catch((error: string) => {
        message(error, { type: "error" });
      });
  }

  // 分页
  function handleSizeChange(val: number) {
    form.page = 1;
    form.itemsPerPage = val;
    onSearch();
  }
  function handleCurrentChange(val: number) {
    form.page = val;
    onSearch();
  }

  // 搜索任务列表
  function onSearch() {
    loading.value = true;
    getJobList(toRaw(form))
      .then(data => {
        dataList.value = data.data;
        pagination.total = data.total;
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

  // 新增、编辑 任务
  function openDialog(title = "新增", row?: JobItemProps) {
    addDialog({
      title: `${title}任务`,
      props: {
        formInline: {
          id: row?.id ?? "",
          name: row?.name ?? "",
          remark: row?.remark ?? "",
          app_name: row?.app_name ?? "",
          apps: apps
        }
      },
      width: "40%",
      draggable: true,
      fullscreenIcon: true,
      closeOnClickModal: false,
      contentRenderer: () => h(editForm, { ref: formRef }),
      beforeSure: (done, { options }) => {
        const FormRef = formRef.value.getRef();
        const curData = options.props.formInline as JobItemProps;
        function chores() {
          message(`您${title}了任务${curData.name}`, {
            type: "success"
          });
          done(); // 关闭弹框
          onSearch(); // 刷新表格数据
        }
        FormRef.validate(valid => {
          if (valid) {
            // 表单规则校验通过
            if (title === "新增") {
              createJob(curData)
                .then(() => {
                  chores();
                })
                .catch(error => {
                  message(error, { type: "error" });
                });
            } else {
              updateJob(curData.id, curData)
                .then(() => {
                  chores();
                })
                .catch(error => {
                  message(error, { type: "error" });
                });
            }
          }
        });
      }
    });
  }
  // 查看日志
  function handleLog(row: JobItemProps) {
    getJobLog(row.id)
      .then((response: Log) => {
        log.value = response;
        logVisible.value = true;
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  }
  // 查询任务的执行结果
  function handleResult(row: JobItemProps) {
    getJobResult(row.id)
      .then(async response => {
        await open(response.html_path);
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  }

  onMounted(() => {
    onSearch();
    getAppTree()
      .then((response: AppTreeResult) => {
        apps.value = response;
        categories.value = response.map(item => item.name);
      })
      .catch(error => {
        message(error, { type: "error" });
      });
  });

  return {
    form,
    loading,
    columns,
    dataList,
    pagination,
    onSearch,
    resetForm,
    openDialog,
    handleRun,
    handleTimer,
    handleConfig,
    handleDelete,
    handleSizeChange,
    handleCurrentChange,
    crontabVisible,
    cronFormData,
    onCancelCron,
    onConfirmCron,
    taskConifgVisible,
    taskConfigData,
    handleConfirmConfig,
    logVisible,
    log,
    handleLog,
    handleResult,
    categories
  };
}
