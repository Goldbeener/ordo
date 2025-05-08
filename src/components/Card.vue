<template>
  <div
    ref="noteCardRef"
    class="w-full h-auto p-[20px] pb-[4px] rounded-xl bg-white overflow-hidden"
  >
    <Editor
      ref="targetElement"
      placeholder="记录日常的待办"
      :content="note.content"
      @update="(content) => updateNote({ id: note.id, content })"
    />
    <div class="toolbar flex h-[36px] items-center justify-end">
      <span class="text-sm mr-auto">{{ formatDate(note.create_time) }}</span>
      <div
        data-html2canvas-ignore
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleCollectNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiStarFill v-if="note.tags" size="16px" color="#f60" />
            <RiStarLine v-else size="16px" />
          </template>
          收藏笔记
        </n-tooltip>
      </div>
      <div
        data-html2canvas-ignore
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleShareNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiShare2Fill size="16px" />
          </template>
          分享卡片
        </n-tooltip>
      </div>
      <div
        data-html2canvas-ignore
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleCopyNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiFileCopyLine size="16px" />
          </template>
          复制笔记文本
        </n-tooltip>
      </div>
      <div
        data-html2canvas-ignore
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleCaptureScreenshot"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiComputerLine size="16px" />
          </template>
          将此笔记设置为壁纸
        </n-tooltip>
      </div>
      <div
        data-html2canvas-ignore
        class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
        @click="handleDeleteNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiDeleteBin6Line size="16px" />
          </template>
          删除笔记
        </n-tooltip>
      </div>
    </div>
  </div>
</template>

<script setup>
import { debounce } from 'lodash-es';
import { format, parseISO } from 'date-fns';
import { useNotification } from 'naive-ui';
import {
  RiFileCopyLine,
  RiComputerLine,
  RiDeleteBin6Line,
  RiStarLine,
  RiStarFill,
  RiShare2Fill,
} from '@remixicon/vue';
import { writeText, writeImage } from '@tauri-apps/plugin-clipboard-manager';

import Editor from './Editor.vue';
import useHandleNote from '../hooks/useHandleNote';
import useHandleCapture from '../hooks/useHandleCapture.js';
import { captureElementToDataUrl } from '../utils/screenshotService.js';

const {
  handleUpdateNote,
  handleMarkNote,
  handleDeleteNote: handleDelete,
} = useHandleNote();
const { targetElement, captureToFile } = useHandleCapture();
const notification = useNotification();

const props = defineProps({
  note: {
    type: Object,
  },
});
const noteCardRef = ref(null);

const updateNote = debounce(handleUpdateNote, 1000);

// 点击设置成壁纸
async function handleCaptureScreenshot() {
  await captureToFile();
  notification.success({
    content: '设置成功！',
    duration: 2000,
  });
}

function dataUriToArrayBuffer(dataUri) {
  // 去掉 Data URI 前缀，如 "data:image/png;base64,"
  const base64 = dataUri.split(',')[1];
  const binaryString = atob(base64); // 解码 Base64 为二进制字符串
  const len = binaryString.length;
  const bytes = new Uint8Array(len);

  for (let i = 0; i < len; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }

  return bytes.buffer; // 返回 ArrayBuffer
}

async function handleShareNote() {
  // 分享卡片
  console.log('目标卡片', noteCardRef.value);
  const imageData = await captureElementToDataUrl(noteCardRef.value);
  await writeImage(dataUriToArrayBuffer(imageData.dataUrl));
  notification.success({
    content: '生成图片成功！',
    duration: 2000,
  });
}

async function handleCopyNote() {
  const editorContent = targetElement.value.getEditorText();
  await writeText(editorContent);
  notification.success({
    content: '复制成功！',
    duration: 2000,
  });
}

async function handleCollectNote() {
  const tags = props.note.tags ? null : ['mark'];
  await handleMarkNote({ ...props.note, tags });
  props.note.tags = tags;
}

async function handleDeleteNote() {
  await handleDelete(props.note.id);
  notification.success({
    content: '删除成功！',
    duration: 2000,
  });
}

function formatDate(date) {
  return format(parseISO(date), 'yyyy-MM-dd HH:mm');
}
</script>

<style lang="less" scoped></style>
