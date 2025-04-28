<template>
  <div class="w-full h-auto p-[20px] pb-[4px] rounded-xl bg-white">
    <Editor
        ref="targetElement"
        placeholder="记录日常的待办"
        :content="note.content"
        @update="(content) => updateNote({ id: note.id, content })"
    />
    <div class="toolbar flex h-[36px] items-center justify-end">
      <span class="text-sm mr-auto">{{ formatDate(note.create_time) }}</span>
      <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
          @click="handleCollectNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiStarFill v-if="note.tags" size="16px" color="#f60"/>
            <RiStarLine v-else size="16px"/>
          </template>
          收藏笔记
        </n-tooltip>
      </div>
      <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
          @click="handleCopyNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiFileCopyLine size="16px"/>
          </template>
          复制笔记文本
        </n-tooltip>
      </div>
      <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
          @click="handleCaptureScreenshot"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiComputerLine size="16px"/>
          </template>
          将此笔记设置为壁纸
        </n-tooltip>
      </div>
      <div
          class="flex justify-center items-center mr-2 rounded-md bg-transparent hover:bg-slate-300 icon-wrapper w-6 h-6"
          @click="handleDeleteNote"
      >
        <n-tooltip trigger="hover">
          <template #trigger>
            <RiDeleteBin6Line size="16px"/>
          </template>
          删除笔记
        </n-tooltip>

      </div>
    </div>
  </div>
</template>

<script setup>
import {debounce} from 'lodash-es';
import {format, parseISO} from 'date-fns';
import {useNotification} from 'naive-ui';
import {
  RiFileCopyLine,
  RiComputerLine,
  RiDeleteBin6Line,
  RiStarLine,
  RiStarFill
} from '@remixicon/vue';
import {writeText} from '@tauri-apps/plugin-clipboard-manager';


import Editor from './Editor.vue';
import useHandleNote from '../hooks/useHandleNote';
import useHandleCapture from '../hooks/useHandleCapture.js'

const {handleUpdateNote, handleMarkNote, handleDeleteNote: handleDelete} = useHandleNote();
const {targetElement, captureToFile} = useHandleCapture();
const notification = useNotification()


const props = defineProps({
  note: {
    type: Object,
  },
});

const updateNote = debounce(handleUpdateNote, 1000);

// 点击设置成壁纸
async function handleCaptureScreenshot() {
  await captureToFile()
  notification.success({
    content: '设置成功！',
    duration: 2000
  })
}

async function handleCopyNote() {
  const editorContent = targetElement.value.getEditorText();
  await writeText(editorContent);
  notification.success({
    content: '复制成功！',
    duration: 2000
  })
}

async function handleCollectNote() {
  const tags = props.note.tags ? null : ['mark'];
  await handleMarkNote({...props.note, tags})
  props.note.tags = tags
}

async function handleDeleteNote() {
  await handleDelete(props.note.id);
  notification.success({
    content: '删除成功！',
    duration: 2000
  })
}


function formatDate(date) {
  return format(parseISO(date), 'yyyy-MM-dd HH:mm');
}
</script>

<style lang="less" scoped></style>
