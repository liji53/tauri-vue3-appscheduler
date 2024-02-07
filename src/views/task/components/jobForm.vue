<script setup lang="ts">
import { ref, reactive } from "vue";
import { JobFormProps } from "../utils/types";

const jobFormRules = reactive({
  app_name: [{ required: true, message: "必须选择一个应用", trigger: "blur" }],
  name: [{ required: true, message: "必须填写任务名称", trigger: "blur" }]
});

const props = withDefaults(defineProps<JobFormProps>(), {
  formInline: () => ({
    name: "",
    remark: "",
    app_name: "",
    apps: []
  })
});
const TreeProps = ref({
  value: "name",
  label: "name",
  children: "children"
});

const ruleFormRef = ref();
const newFormInline = ref(props.formInline);

function getRef() {
  return ruleFormRef.value;
}
const onSelectChnage = (value: string) => {
  for (const app of newFormInline.value.apps) {
    if ("children" in app) {
      for (const child of app["children"]) {
        if (child.name === value) {
          newFormInline.value.name = child.name;
          console.log(newFormInline.value.name);
        }
      }
    } else {
      if (app.name === value) {
        newFormInline.value.name = app.name;
        console.log(newFormInline.value.name);
      }
    }
  }
};

defineExpose({ getRef });
</script>

<template>
  <el-form
    ref="ruleFormRef"
    :model="newFormInline"
    :rules="jobFormRules"
    label-width="82px"
  >
    <el-form-item label="应用" prop="app_name">
      <el-tree-select
        v-model="newFormInline.app_name"
        :props="TreeProps"
        :data="newFormInline.apps"
        :render-after-expand="false"
        placeholder="请选择应用"
        clearable
        @change="onSelectChnage"
      />
    </el-form-item>
    <el-form-item label="任务名称" prop="name">
      <el-input
        v-model="newFormInline.name"
        clearable
        placeholder="请输入任务名称"
      />
    </el-form-item>
    <el-form-item label="备注">
      <el-input
        v-model="newFormInline.remark"
        placeholder="请输入备注信息"
        type="textarea"
      />
    </el-form-item>
  </el-form>
</template>
