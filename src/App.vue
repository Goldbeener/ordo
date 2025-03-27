<template>
  <div class="w-screen h-screen">
    <header class="bg-white h-[36px] rounded-md">
      <div class="h-full flex items-center">
        <div class="grow"></div>
        <div
          class="flex justify-center items-center rounded-md bg-transparent w-[36px] h-full"
        >
          <RiAddLine size="24px" />
        </div>
      </div>
    </header>
    <main class="containers flex grow mt-3">
      <div class="w-full p-[32px] rounded-lg overflow-hidden">
        <Card />
      </div>
    </main>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import Card from './components/Card.vue';
// import TopBar from './components/TopBar.vue';
import { ref } from 'vue';

const enableTop = ref(true);

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

<style lang="less" scoped></style>
