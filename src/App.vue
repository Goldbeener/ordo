<template>
  <div class="w-screen h-screen">
    <header class="bg-white px-3 h-[56px] rounded-md">
      <TopBar />
    </header>
    <main class="containers flex grow mt-3">
      <div class="w-full py-[32px] px-[16px] rounded-lg overflow-hidden">
        <Card />
      </div>
    </main>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import Card from './components/Card.vue';
import TopBar from './components/TopBar.vue';

const enableTop = ref(true);
const showCards = ref([]);

getTodayNotes();

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

async function getTodayNotes() {
  try {
    const notes = await invoke('get_notes_by_time_range_command', {
      startTime: '2025-03-28 00:00:00',
      endTime: '2025-03-28 23:59:59',
    });
    console.log('获取笔记成功', notes);
  } catch (error) {
    console.log('获取笔记失败', error);
  }
}
</script>

<style lang="less"></style>
