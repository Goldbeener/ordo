<template>
  <main class="container w-screen overflow-scroll">
    <div
      class="w-full py-[32px] px-[16px] pb-0"
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

const { noteList, handleGetTodayNotes } = useHandleNote();

handleGetTodayNotes();
</script>

<style scoped lang="less">

</style>