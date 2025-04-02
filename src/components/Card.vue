<template>
  <div  class="w-full h-auto p-[20px] pb-[4px] rounded-xl bg-white">
    <Editor
        ref="targetElement"
      :content="note.content"
      @update="(content) => updateNote({ id: note.id, content })"
    />
    <div class="toolbar flex h-[36px] items-center justify-end">
      <span class="text-sm mr-auto">{{ formatDate(note.create_time) }}</span>
      <div
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
      >
        <RiFileCopyLine size="16px" />
      </div>
      <div
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleCaptureScreenshot"
      >
        <RiComputerLine size="16px" />
      </div>
      <div
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleDeleteNote(note.id)"
      >
        <RiDeleteBin6Line size="16px" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { debounce } from 'lodash-es';
import { format, parseISO } from 'date-fns';
import {
  RiFileCopyLine,
  RiComputerLine,
  RiDeleteBin6Line,
} from '@remixicon/vue';

import Editor from './Editor.vue';
import useHandleNote from '../hooks/useHandleNote';
import useHandleCapture from '../hooks/useHandleCapture.js'

const { handleUpdateNote, handleDeleteNote } = useHandleNote();
const { targetElement,  captureToFile } = useHandleCapture();

const _props = defineProps({
  note: {
    type: Object,
  },
});

const updateNote = debounce(handleUpdateNote, 1000);

// 点击设置成壁纸
function handleCaptureScreenshot() {
  console.log('获取笔记截图Ref', targetElement.value.$el)
  captureToFile()
}




function formatDate(date) {
  return format(parseISO(date), 'yyyy-MM-dd HH:mm');
}
</script>

<style lang="less" scoped></style>
