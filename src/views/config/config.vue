<script setup lang="ts">
import { reactive, ref, onMounted } from "vue";
import { useRenderIcon } from "@/components/ReIcon/src/hooks";
import Lock from "@iconify-icons/ri/lock-fill";
import { getConfig, setConfig } from "@/api/config";

import type { FormRules } from "element-plus";
import { message } from "@/utils/message";

defineOptions({
  name: "Config"
});
const formRef = ref();
const form = ref({
  apps_url: "",
  app_user: "",
  app_passwd: ""
});

const rules = reactive<FormRules>({
  apps_url: [
    { required: true, message: "应用商城地址为必填项", trigger: "blur" },
    { type: "url", message: "地址格式不正确", trigger: "blur" }
  ]
});

const resetForm = formEl => {
  if (!formEl) return;
  getConfig().then(response => {
    form.value = response;
  });
};
const submitForm = async formEl => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      await setConfig(form.value);
      message("设置成功", { type: "success" });
    } else {
      console.log("error submit!", fields);
    }
  });
};

onMounted(() => {
  getConfig().then(response => {
    form.value = response;
  });
});
</script>

<template>
  <div class="main">
    <el-card>
      <el-form :model="form" label-width="120px" :rules="rules" ref="formRef">
        <el-form-item label="应用商城地址" prop="apps_url">
          <el-input
            v-model="form.apps_url"
            placeholder="请输入应用商城的地址"
            clearable
          />
        </el-form-item>

        <el-form-item label="安装应用的账号" prop="app_user">
          <el-input
            v-model="form.app_user"
            placeholder="安装应用时, svn/git的账号"
            clearable
          />
        </el-form-item>

        <el-form-item label="安装应用的密码" prop="app_passwd">
          <el-input
            show-password
            v-model="form.app_passwd"
            placeholder="安装应用时, svn/git的账号密码"
            clearable
            :prefix-icon="useRenderIcon(Lock)"
          />
        </el-form-item>

        <el-form-item>
          <el-button @click="resetForm(formRef)">重置</el-button>
          <el-button type="primary" @click="submitForm(formRef)">
            保存
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>
