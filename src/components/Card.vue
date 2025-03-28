<template>
  <div class="card-wrapper w-full h-auto p-[20px] pb-[4px] rounded-xl bg-white">
    <Editor @update="handleUpdateNote" />
    <div class="toolbar flex h-[36px] items-center justify-between">
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
        <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        >
          <RiDeleteBin6Line size="16px" />
        </div>
      </div>
      <span class="text-sm">{{ formatDate(note.create_time) }}</span>
    </div>
  </div>
</template>

<script setup>
import Editor from './Editor.vue';
import {
  RiFileCopyLine,
  RiComputerLine,
  RiDeleteBin6Line,
} from '@remixicon/vue';
import { format, parseISO } from 'date-fns';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  note: {
    type: Object,
  },
});

/**
 * todo
 * 1. 节流
 * 2. v-model
 * */
async function handleUpdateNote(editorValue) {
  console.log('update', editorValue);
  try {
    const notes = await invoke('update_note', {
      id: props.note.id,
      title: '',
      content: editorValue,
    });
    console.log('更新笔记成功', notes);
  } catch (error) {
    console.log('更新笔记成功', error);
  }
}

function formatDate(date) {
  return format(parseISO(date), 'yyyy-MM-dd HH:mm');
}
</script>

<style lang="less" scoped></style>
