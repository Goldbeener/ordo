<template>
  <div class="top-bar h-full flex items-center">
    <n-tabs type="segment" animated>
      <n-tab-pane name="chap1" tab="Today"></n-tab-pane>
      <n-tab-pane name="chap2" tab="Week"></n-tab-pane>
      <n-tab-pane name="chap3" tab="Mine"></n-tab-pane>
    </n-tabs>
    <div
      class="flex justify-center items-center rounded-md bg-transparent w-[36px] h-full"
    >
      <RiAddLine size="24px" />
    </div>
  </div>
</template>

<script setup>
import { RiAddLine } from '@remixicon/vue';
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
