<template>
  <n-notification-provider>
    <div data-tauri-drag-region class="w-full h-7 cursor-move text-center leading-7 font-bold gradient-text">
      Ordo
    </div>
  <div class="w-screen h-screen flex flex-col items-center">
    <header class="w-full bg-white px-3 h-[56px] rounded-md">
      <TopBar @create-note="handleCreateNote" @switch="(idx) => currentIndex = idx"/>
    </header>
    <component :is="currentComp"></component>
  </div>
  </n-notification-provider>

</template>

<script setup>
import TopBar from './components/TopBar.vue';
import Today from "./components/Today.vue";
import Weekly from "./components/Weekly.vue";
import Mine from "./components/Mine.vue"

import useHandleNote from './hooks/useHandleNote';
const currentIndex = ref(0);
const compList = [Today, Weekly, Mine];

const currentComp = computed(() => compList[currentIndex.value]);

const { handleCreateNote } = useHandleNote();
</script>

<style lang="less">
  ::-webkit-scrollbar {
    display: none;
  }
  .gradient-text {
    --n-bezier: cubic-bezier(.4, 0, .2, 1);
    --n-rotate: 252deg;
    --n-color-start: rgba(240, 160, 32, 0.6);
    --n-color-end: #f0a020;
    -webkit-background-clip: text;
    background-clip: text;
    color: #0000;
    white-space: nowrap;
    background-image: linear-gradient(var(--n-rotate), var(--n-color-start) 0%, var(--n-color-end) 100%);
    transition: --n-color-start .3s var(--n-bezier), --n-color-end .3s var(--n-bezier);
  }
</style>
