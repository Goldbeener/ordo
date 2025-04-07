<template>
  <main class="container w-screen overflow-scroll">
    <div
      class="w-full py-8 px-4 pb-0"
      v-for="note in noteList"
      :key="note.id"
    >
      <Card :note="note" />
    </div>
  </main>
</template>

<script setup>
import Card from "./Card.vue";
import useHandleNote from '../hooks/useHandleNote';

const { noteList, handleGetTodayNotes, handleCreateNote } = useHandleNote();

init();
async function init(){
  await handleGetTodayNotes();
  console.log('????今日数据', noteList.value.length)
  if(!noteList.value.length){
    // 如果没有数据，创建一个新的空白的
    handleCreateNote()
  }
}

</script>

<style scoped lang="less">

</style>