<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { open } from "@tauri-apps/api/dialog";
import infoFilled from "@iconify-icons/ep/info-filled";
import fileAdd from "@iconify-icons/ep/document-add";
import { useRenderIcon } from "@/components/ReIcon/src/hooks";

import {
  ElInput,
  ElSwitch,
  ElRadioGroup,
  ElCheckboxGroup,
  ElDatePicker,
  ElTimeSelect,
  ElInputNumber,
  ElSlider,
  ElSelect
} from "element-plus";
import { hasOwnProp } from "@pureadmin/utils";

type ControlType =
  | "Text"
  | "TextArea"
  | "Switch"
  | "Radio"
  | "CheckBox"
  | "Date"
  | "Time"
  | "InputNumber"
  | "Slider"
  | "Selected"
  | "Selecteds"
  | "Dir"
  | "Dirs"
  | "File"
  | "Files";

interface ControlData {
  default?: any;
  fieldName: string;
  [key: string]: any;
}

interface Control {
  ControlType: ControlType;
  id: string;
  data: ControlData;
}

const props = defineProps<{
  formJson: Control[];
}>();
defineEmits(["confirm-config"]);

const dynFormRef = ref();
const formData = ref({});
const rules = ref({});

const formControls = computed(() => props.formJson);

function getComponentType(controlType: ControlType): any {
  const componentMap: Record<ControlType, any> = {
    Text: ElInput,
    TextArea: ElInput,
    Switch: ElSwitch,
    Radio: ElRadioGroup,
    CheckBox: ElCheckboxGroup,
    Date: ElDatePicker,
    Time: ElTimeSelect,
    InputNumber: ElInputNumber,
    Slider: ElSlider,
    Selected: ElSelect,
    Selecteds: ElSelect,
    Dir: "",
    Dirs: "",
    File: "",
    Files: ""
  };
  return componentMap[controlType];
}

function getComponentProps(control: Control): Record<string, any> {
  const props = {
    placeholder: control.data.placeholder,
    type: getInputType(control.ControlType),
    multiple: control.ControlType === "Selecteds" || undefined,
    end: control.ControlType === "Time" ? "23:30" : undefined,
    min: control.ControlType === "Slider" ? control.data.min : undefined,
    max: control.ControlType === "Slider" ? control.data.max : undefined
  };

  return props;
}

function getInputType(controlType: ControlType): string {
  const inputTypeMap: Record<ControlType, string> = {
    Text: "text",
    TextArea: "textarea",
    Switch: "checkbox",
    Radio: "radio",
    CheckBox: "checkbox",
    Date: "date",
    Time: "time",
    InputNumber: "number",
    Slider: "range",
    Selected: "",
    Selecteds: "",
    Dir: "",
    Dirs: "",
    File: "",
    Files: ""
  };
  return inputTypeMap[controlType];
}

function getFormListRules(rules: any[]) {
  const result: any[] = [];
  if (Array.isArray(rules) && rules && rules.length > 0) {
    rules.forEach(item => {
      if (item.type == "enum") {
        const func = eval(`(${item.value})`);
        result.push({
          validator: func,
          trigger: "blur"
        });
      } else if (item.type == "func") {
        const mainData = formData.value;
        const func = eval(
          `((rule, value, callback, mainData = mainData) => {${item.value.func}})`
        );
        result.push({
          validator: func,
          trigger: "blur"
        });
        console.log("mainData", mainData);
      } else if (item.type == "high") {
        if (item.value.ruleType == 5) {
          result.push({
            validator: eval(item.value.validor),
            trigger: item.value.trigger
          });
          return;
        }
        // let high = JSON.parse(JSON.stringify(item.value));
        // delete high.ruleType;
        result.push(item.value);
      }
    });
  }
  return result;
}

function getRules(item: Control) {
  let rule: any[] = [];
  if (item.data.required) {
    rule.push({
      required: true,
      message: "请输入" + item.data.label,
      trigger: "blur"
    });
  }
  rule = rule.concat(getFormListRules(item.data.rule));
  rules.value[item.data.fieldName] = rule;
}

const is_select_componet = (controlType: ControlType) => {
  return ["Radio", "CheckBox", "Selected", "Selecteds"].includes(controlType);
};

const select_path = async (
  control_type: string,
  field_name: string,
  extensions: any[]
) => {
  let multiple = false;
  if (control_type === "Dirs" || control_type === "Files") {
    multiple = true;
  }

  let selected;
  if (control_type === "Dir" || control_type === "Dirs") {
    selected = await open({
      directory: true,
      multiple: multiple
    });
  } else {
    const file_extensions = extensions.map(item => item.value);
    selected = await open({
      multiple: multiple,
      filters: [
        {
          name: "文件类型",
          extensions: file_extensions
        }
      ]
    });
  }
  if (multiple) {
    formData.value[field_name] = JSON.stringify(selected);
  } else {
    formData.value[field_name] = selected;
  }
};

onMounted(() => {
  props.formJson.forEach(control => {
    /// 表单的默认值
    const defaultValue = control.data.default;
    if (is_select_componet(control.ControlType)) {
      formData.value[control.data.fieldName] = control.data.itemConfig.value;
    } else {
      formData.value[control.data.fieldName] = defaultValue;
    }

    /// 表单校验规则
    getRules(control);
  });
});
</script>

<template>
  <div>
    <el-form
      ref="dynFormRef"
      @submit.prevent="$emit('confirm-config', dynFormRef, formData)"
      :model="formData"
      :rules="rules"
      label-position="right"
      label-width="180px"
    >
      <el-form-item
        v-for="control in formControls"
        :key="control.id"
        :label="control.data.label"
        :prop="control.data.fieldName"
      >
        <!-- 加入lable后面的提示信息 -->
        <template
          v-if="control.data.tip != '' && control.data.tip != null"
          #label
        >
          {{ control.data.label }}
          <el-tooltip
            class="item"
            effect="dark"
            :content="control.data.tip"
            placement="top"
          >
            <IconifyIconOffline :icon="infoFilled" />
          </el-tooltip>
        </template>

        <el-row
          v-if="['Dir', 'Dirs', 'File', 'Files'].includes(control.ControlType)"
        >
          <el-col :span="20">
            <el-input v-model="formData[control.data.fieldName]" />
          </el-col>
          <el-col :span="4">
            <el-button
              type="primary"
              :icon="useRenderIcon(fileAdd)"
              @click="
                select_path(
                  control.ControlType,
                  control.data.fieldName,
                  hasOwnProp(control.data.itemConfig, 'items')
                    ? control.data.itemConfig.items
                    : []
                )
              "
            />
          </el-col>
        </el-row>
        <component
          v-else-if="control.ControlType != 'Date'"
          :is="getComponentType(control.ControlType)"
          v-model="formData[control.data.fieldName]"
          v-bind="getComponentProps(control)"
        >
          <!-- 对于checkbox、select等选择组件 渲染选项 -->
          <template v-if="is_select_componet(control.ControlType)">
            <template v-for="item in control.data.itemConfig.items">
              <el-option
                v-if="
                  control.ControlType === 'Selected' ||
                  control.ControlType === 'Selecteds'
                "
                :key="item.id"
                :label="item.label"
                :value="item.value"
              />
              <el-checkbox
                v-else-if="control.ControlType === 'CheckBox'"
                :label="item.value"
                :value="item.value"
                :key="item.id + '1'"
              />
              <el-radio
                v-else-if="control.ControlType === 'Radio'"
                :label="item.value"
                :key="item.id + '2'"
              >
                {{ item.label }}
              </el-radio>
            </template>
          </template>
        </component>

        <!-- 使用component不知道为什么不能显示日期 -->
        <el-date-picker
          v-else
          v-model="formData[control.data.fieldName]"
          type="date"
          value-format="YYYY-MM-DD"
        />
      </el-form-item>
      <el-button type="primary" native-type="submit">确认</el-button>
    </el-form>
  </div>
</template>
