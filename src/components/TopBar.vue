<template>
  <div class="top-bar h-full flex items-center">
    <n-tabs type="segment" animated @before-leave="handleSwitch">
      <n-tab-pane :name="0" tab="今日"></n-tab-pane>
      <n-tab-pane :name="1" tab="本周"></n-tab-pane>
      <n-tab-pane :name="2" tab="我的"></n-tab-pane>
    </n-tabs>
    <div
      class="flex justify-center items-center rounded-md bg-transparent w-[36px] h-[40px] hover:bg-slate-300 ml-2 cursor-pointer"
      :class="{ disabled: activeTab }"
      @click="emit('create-note')"
    >
      <RiAddLine size="24px" />
    </div>
  </div>
</template>

<script setup>
import { RiAddLine } from '@remixicon/vue';

const emit = defineEmits(['create-note', 'switch']);

const activeTab = ref(0);

function handleSwitch(tabName) {
  activeTab.value = tabName;
  emit('switch', tabName)
  return true
}
</script>

<style lang="less" scoped>
.top-bar {
  & :deep(.n-tabs) {
    --n-color-segment: #ffebcd !important;
  }

  & :deep(.n-tabs-pane-wrapper) {
    display: none;
  }
  .disabled {
    pointer-events: none;
    cursor: not-allowed;
  }
}
</style>
