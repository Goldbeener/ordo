<template>
  <div class="top-bar h-full flex items-center">
    <n-tabs type="segment" animated>
      <n-tab-pane name="chap1" tab="今日"></n-tab-pane>
      <n-tab-pane name="chap2" tab="本周"></n-tab-pane>
      <n-tab-pane name="chap3" tab="我的"></n-tab-pane>
    </n-tabs>
    <div
      class="flex justify-center items-center rounded-md bg-transparent w-[36px] h-[40px] hover:bg-slate-300 ml-2 cursor-pointer"
      @click="emit('create-note')"
    >
      <RiAddLine size="24px" />
    </div>
  </div>
</template>

<script setup>
import { RiAddLine } from '@remixicon/vue';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits(['create-note']);
</script>

<style lang="less" scoped>
.top-bar {
  & :deep(.n-tabs) {
    --n-color-segment: #ffebcd !important;
  }

  & :deep(.n-tabs-pane-wrapper) {
    display: none;
  }
}
</style>
