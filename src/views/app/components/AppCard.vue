<script setup lang="ts">
import { ref, computed } from "vue";
import More2Fill from "@iconify-icons/ri/more-2-fill";
import { MdPreview } from "md-editor-v3";
import "md-editor-v3/lib/preview.css";

const props = defineProps(["app", "readme"]);
defineEmits(["install-app", "upgrade-app", "uninstall-app", "readme-app"]);

const infoDialogVisible = ref(false);

const tag_color = computed(() => {
  return props.app.status === "已安装" ? "#90EE90" : "#00BFFF";
});

// 卡片的样式
const cardClass = computed(() => [
  "list-card-item",
  {
    "list-card-item__disabled": props.app.status === "已安装" ? false : true
  }
]);
</script>

<template>
  <div :class="cardClass">
    <div class="list-card-item_detail bg-bg_color">
      <!-- logo + 状态 + 操作 -->
      <el-row justify="end">
        <div class="list-card-item_detail--operation">
          <el-tag
            type="info"
            effect="dark"
            class="mx-1 list-card-item_detail--operation--tag"
          >
            {{ app.category }}
          </el-tag>
          <el-tag
            :color="tag_color"
            effect="dark"
            class="mx-1 list-card-item_detail--operation--tag"
          >
            {{ app.status }}
          </el-tag>
          <el-dropdown trigger="click">
            <IconifyIconOffline :icon="More2Fill" class="text-[24px]" />
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item
                  v-if="app.status === '未安装'"
                  @click="$emit('install-app', app)"
                >
                  安装
                </el-dropdown-item>
                <el-dropdown-item
                  v-if="app.status === '已安装'"
                  @click="$emit('upgrade-app', app)"
                >
                  升级
                </el-dropdown-item>
                <el-dropdown-item
                  v-if="app.status === '已安装'"
                  @click="$emit('uninstall-app', app)"
                >
                  卸载
                </el-dropdown-item>
                <el-dropdown-item
                  @click="
                    infoDialogVisible = true;
                    $emit('readme-app', app);
                  "
                >
                  详情
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-row>

      <!-- name -->
      <el-row>
        <p class="list-card-item_detail--name text-text_color_primary">
          {{ app.name }}
        </p>
      </el-row>

      <p class="list-card-item_detail--desc text-text_color_regular">
        {{ app.description }}
      </p>
    </div>
  </div>

  <!-- dialog -->
  <el-dialog v-model="infoDialogVisible" title="readme">
    <MdPreview
      :editorId="'preview-only'"
      :modelValue="readme"
      style="padding: auto"
    />
  </el-dialog>
</template>

<style scoped lang="scss">
.list-card-item {
  display: flex;
  flex-direction: column;
  margin-bottom: 12px;
  overflow: hidden;
  cursor: pointer;
  border-radius: 3px;

  &_detail {
    flex: 1;
    min-height: 140px;
    padding: 24px 32px;

    &--logo {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 56px;
      height: 56px;
      font-size: 32px;
      color: #0052d9;
      background: #e0ebff;
      border-radius: 50%;
      background-size: cover;
      background-position: center;

      &__disabled {
        color: #a1c4ff;
      }
    }

    &--operation {
      display: flex;
      height: 100%;

      &--tag {
        border: 0;
      }
    }

    &--name {
      margin: 24px 0 8px;
      font-size: 16px;
      font-weight: 400;
    }

    &--desc {
      display: -webkit-box;
      height: 40px;
      margin-bottom: 24px;
      overflow: hidden;
      font-size: 12px;
      line-height: 20px;
      text-overflow: ellipsis;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
    }
  }

  &__disabled {
    .list-card-item_detail--name,
    .list-card-item_detail--desc {
      color: var(--el-text-color-disabled);
    }

    .list-card-item_detail--operation--tag {
      color: #bababa;
    }
  }
}
</style>
