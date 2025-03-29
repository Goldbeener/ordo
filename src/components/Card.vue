<template>
  <div class="card-wrapper w-full h-auto p-[20px] pb-[4px] rounded-xl bg-white">
    <Editor
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
import useHandleNote from '../hooks/useHandleNote';
import {
  RiFileCopyLine,
  RiComputerLine,
  RiDeleteBin6Line,
} from '@remixicon/vue';

import Editor from './Editor.vue';

const { handleUpdateNote, handleDeleteNote } = useHandleNote();

const props = defineProps({
  note: {
    type: Object,
  },
});

const updateNote = debounce(handleUpdateNote, 1000);

function formatDate(date) {
  return format(parseISO(date), 'yyyy-MM-dd HH:mm');
}
</script>

<style lang="less" scoped></style>
