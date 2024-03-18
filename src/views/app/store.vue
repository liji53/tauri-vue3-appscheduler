<script setup lang="ts">
import { ref } from "vue";
import { useApp } from "./utils/hookStore";
import AppCard from "./components/AppCard.vue";
import { useRenderIcon } from "@/components/ReIcon/src/hooks";
import Search from "@iconify-icons/ep/search";
import Refresh from "@iconify-icons/ep/refresh";

defineOptions({
  name: "Store"
});

const formRef = ref();
const {
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
} = useApp();
</script>

<template>
  <div class="main">
    <el-form
      ref="formRef"
      :inline="true"
      :model="form"
      class="search-form bg-bg_color w-[99/100] pl-8 pt-[12px]"
    >
      <el-form-item label="应用名称：" prop="appName">
        <el-input
          v-model="form.appName"
          placeholder="请输入应用名称"
          clearable
          class="!w-[160px]"
        />
      </el-form-item>
      <el-form-item label="应用分类：" prop="category">
        <el-select
          v-model="form.category"
          placeholder="请选择应用分类"
          clearable
          class="!w-[180px]"
        >
          <el-option
            v-for="(item, index) in categories"
            :key="index"
            :value="item"
            :label="item"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="状态：" prop="status">
        <el-select
          v-model="form.status"
          placeholder="请选择状态"
          clearable
          class="!w-[120px]"
        >
          <el-option label="已安装" :value="'已安装'" />
          <el-option label="未安装" :value="'未安装'" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button
          type="primary"
          :icon="useRenderIcon(Search)"
          :loading="loading"
          @click="onSearch"
        >
          搜索
        </el-button>
        <el-button :icon="useRenderIcon(Refresh)" @click="resetForm(formRef)">
          重置
        </el-button>
      </el-form-item>
    </el-form>

    <div v-loading="loading" style="margin-top: 12px">
      <el-empty description="暂无应用" v-show="pagination.total === 0" />
      <template v-if="pagination.total > 0">
        <el-row :gutter="16">
          <el-col
            v-for="app of apps"
            :key="app.name"
            :xs="24"
            :sm="12"
            :md="8"
            :lg="6"
            :xl="4"
          >
            <AppCard
              :app="app"
              :readme="readme"
              @install-app="handleInstallApp"
              @uninstall-app="handleUninstallApp"
              @upgrade-app="handleUpgradeApp"
              @readme-app="handleReadmeApp"
            />
          </el-col>
        </el-row>

        <el-pagination
          class="float-right"
          v-model:currentPage="pagination.currentPage"
          :page-size="pagination.pageSize"
          :total="pagination.total"
          :page-sizes="[12, 24, 36]"
          background
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="onPageSizeChange"
          @current-change="onCurrentPageChange"
        />
      </template>
    </div>
  </div>
</template>

<style scoped lang="scss">
.search-form {
  :deep(.el-form-item) {
    margin-bottom: 12px;
  }
}
</style>
