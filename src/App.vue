<template>
  <div class="w-screen h-screen flex flex-col items-center">
    <header class="w-full bg-white px-3 h-[56px] rounded-md">
      <TopBar @create-note="handleCreateNote" />
    </header>
    <main class="container w-screen overflow-scroll">
      <div
        class="w-full py-[32px] px-[16px] pb-0"
        v-for="note in noteList"
        :key="note.id"
      >
        <Card :note="note" />
      </div>
    </main>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import Card from './components/Card.vue';
import TopBar from './components/TopBar.vue';

import useHandleNote from './hooks/useHandleNote';

const enableTop = ref(true);

const { noteList, handleCreateNote, handleGetTodayNotes } = useHandleNote();

handleGetTodayNotes();

async function changeWallpaper() {
  try {
    const res = await invoke('set_wallpaper');
    console.log('壁纸更新成功', res);
  } catch (error) {
    console.log('壁纸更新失败', error);
  }
}

async function handleToggleLayer() {
  enableTop.value = !enableTop.value;
  try {
    await invoke('toggle_always_on_top', { enable: enableTop.value });
    console.log('置顶成功');
  } catch (error) {
    console.log('置顶失败', error);
  }
}
</script>

<style lang="less">
  ::-webkit-scrollbar {
    display: none;
  }
</style>
