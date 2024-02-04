<script setup lang="ts">
import Search from "../search/index.vue";
import Notice from "../notice/index.vue";
import SidebarItem from "./sidebarItem.vue";
import { isAllEmpty } from "@pureadmin/utils";
import { ref, nextTick, computed } from "vue";
import { useNav } from "@/layout/hooks/useNav";
import { usePermissionStoreHook } from "@/store/modules/permission";
import Setting from "@iconify-icons/ri/settings-3-line";

const menuRef = ref();

const { route, title, backTopMenu, onPanel } = useNav();

const defaultActive = computed(() =>
  !isAllEmpty(route.meta?.activePath) ? route.meta.activePath : route.path
);

nextTick(() => {
  menuRef.value?.handleResize();
});
</script>

<template>
  <div
    v-loading="usePermissionStoreHook().wholeMenus.length === 0"
    class="horizontal-header"
  >
    <div class="horizontal-header-left" @click="backTopMenu">
      <img src="/logo.svg" alt="logo" />
      <span>{{ title }}</span>
    </div>
    <el-menu
      router
      ref="menuRef"
      mode="horizontal"
      class="horizontal-header-menu"
      :default-active="defaultActive"
    >
      <sidebar-item
        v-for="route in usePermissionStoreHook().wholeMenus"
        :key="route.path"
        :item="route"
        :base-path="route.path"
      />
    </el-menu>
    <div class="horizontal-header-right">
      <!-- 菜单搜索 -->
      <Search />
      <!-- 通知 -->
      <Notice id="header-notice" />
      <span
        class="set-icon navbar-bg-hover"
        title="打开项目配置"
        @click="onPanel"
      >
        <IconifyIconOffline :icon="Setting" />
      </span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-loading-mask) {
  opacity: 0.45;
}

.logout {
  max-width: 120px;

  ::v-deep(.el-dropdown-menu__item) {
    display: inline-flex;
    flex-wrap: wrap;
    min-width: 100%;
  }
}
</style>
