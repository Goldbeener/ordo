<template>
  <n-notification-provider>
    <div ref="containerRef" v-show="!isCollapsed">
      <div data-tauri-drag-region class="w-full h-7 cursor-move text-center leading-7 font-bold gradient-text">
        Ordo
      </div>
      <div  class="w-screen px-4 h-screen flex flex-col items-center">
        <header class="w-full bg-white px-3 h-[56px] rounded-md">
          <TopBar @create-note="handleCreateNote" @switch="(idx) => currentIndex = idx"/>
        </header>
        <component :is="currentComp"></component>
      </div>
    </div>
  </n-notification-provider>

  <!--  折叠图标 -->
    <div class="toggle-wrapper w-5 h-28 bg-yellow-400 rounded-lg fixed -left-1 top-1/2 -translate-y-1/2 flex items-center"
         @click="handleToggleCollapse">
      <RiArrowRightDoubleFill v-if="!isCollapsed" size="24"/>
      <RiArrowLeftDoubleFill v-else size="24" />
    </div>

</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { useMotion } from '@vueuse/motion'


import TopBar from './components/TopBar.vue';
import Today from './components/Today.vue';
import Weekly from './components/Weekly.vue';
import Mine from "./components/Mine.vue"

import { RiArrowRightDoubleFill, RiArrowLeftDoubleFill } from '@remixicon/vue';

import useHandleNote from './hooks/useHandleNote';

const isCollapsed = ref(false);
const currentIndex = ref(0);
const compList = [Today, Weekly, Mine];
const containerRef = ref(null);

const currentComp = computed(() => compList[currentIndex.value]);

const { handleCreateNote } = useHandleNote();


const { apply, stop } =  useMotion(containerRef, {
  initial: {
    opacity: 0,
    x: 400, // 初始从右侧偏移
  },
  visible: {
    opacity: 1,
    x: 0,
    transition: {
      type: 'spring',
      stiffness: 250,
      damping: 20,
      mass: 0.5
    },
  },
})

async function handleToggleCollapse() {
  invoke(isCollapsed.value  ?  'expand_window' : 'collapse_window');
  isCollapsed.value = !isCollapsed.value;
  // 元素显示后再执行动画
  if (!isCollapsed.value) {
    await nextTick()
    // await apply('initial')
    await  apply('visible')
  } else {
    stop()
  }
}
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
