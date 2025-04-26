<template>
  <main class="container w-screen pt-8 px-4 box-border overflow-hidden">
    <div class="card-wrapper w-full h-full overflow-scroll">
      <Card class="mb-4" v-for="note in noteList" :key="note.id" :note="note"/>
    </div>
  </main>
</template>

<script setup>
import Card from "./Card.vue";
import useHandleNote from '../hooks/useHandleNote';

const {noteList, handleGetTodayNotes, handleCreateNote} = useHandleNote();

init();

async function init() {
  await handleGetTodayNotes();
  if (!noteList.value.length) {
    // 如果没有数据，创建一个新的空白的
    handleCreateNote()
  }
}

</script>

<style scoped lang="less">
</style>