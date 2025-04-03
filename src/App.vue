<template>
  <n-notification-provider>
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
</style>
