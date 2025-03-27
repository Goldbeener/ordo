<template>
  <div class="card-wrapper w-full h-auto p-[20px] pb-[0] rounded-xl bg-white">
    <Editor />
    <div class="toolbar flex h-[32px] items-center justify-between">
      <div class="flex items-center">
        <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        >
          <RiFileCopyLine size="16px" />
        </div>
        <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        >
          <RiComputerLine size="16px" />
        </div>
      </div>
      <span class="text-sm">2023/3/2 11:13 AM</span>
    </div>
  </div>
</template>

<script setup>
import Editor from './Editor.vue';
import { RiFileCopyLine, RiComputerLine } from '@remixicon/vue';
</script>

<style lang="less" scoped></style>
