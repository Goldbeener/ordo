<template>
  <main
    class="container w-screen pt-8 px-4 box-border overflow-hidden relative"
  >
    <div
      class="card-wrapper w-full h-full overflow-scroll"
      v-infinite-scroll="[onLoadMore, { distance: 10, canLoadMore }]"
    >
      <Card class="mb-4" v-for="note in noteList" :key="note.id" :note="note" />
    </div>
  </main>
</template>

<script setup>
import { vInfiniteScroll } from '@vueuse/components';
import Card from './Card.vue';
import useHandleNote from '../hooks/useHandleNote';

const { noteList, finished, handleGetTodayNotes, handleCreateNote } =
  useHandleNote();

init();

async function init() {
  await handleGetTodayNotes();
  if (!noteList.value.length) {
    // 如果没有数据，创建一个新的空白的
    handleCreateNote();
  }
}

function onLoadMore() {
  // 滚动加载更多
  console.log('滚动加载更多');
  handleGetTodayNotes();
}

function canLoadMore() {
  // inidicate when there is no more content to load so onLoadMore stops triggering
  return !finished.value;
}
</script>

<style scoped lang="less">
.filter-btn {
  width: 120px !important;
}
</style>
