<template>
  <n-notification-provider>
    <div
      ref="containerRef"
      v-show="!isCollapsed"
      class="h-[100vh] overflow-hidden flex flex-col"
    >
      <div
        data-tauri-drag-region
        class="w-full h-7 cursor-move text-center leading-7 font-bold gradient-text"
      >
        Ordo
      </div>
      <div
        class="w-screen flex-auto overflow-hidden px-4 flex flex-col items-center"
      >
        <header class="w-full bg-white px-3 h-[56px] rounded-md">
          <TopBar
            @create-note="handleCreateNote"
            @switch="(idx) => (currentIndex = idx)"
          />
        </header>
        <component :is="currentComp" class="flex-auto"></component>
      </div>
    </div>
  </n-notification-provider>

  <!--  折叠图标 -->
  <div
    v-show="showCollapseIcon"
    class="toggle-wrapper w-4 h-28 bg-yellow-400 rounded-lg fixed left-0 top-1/2 -translate-y-1/2 flex items-center"
    @click="handleToggleCollapse"
  >
    <RiArrowRightDoubleFill v-if="!isCollapsed" size="24" />
    <RiArrowLeftDoubleFill v-else size="24" />
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { useMotion } from '@vueuse/motion';
import { RiArrowRightDoubleFill, RiArrowLeftDoubleFill } from '@remixicon/vue';

import TopBar from './components/TopBar.vue';
import Today from './components/Today.vue';
// import StaredNotes from "./components/StaredNotes.vue";
import Weekly from './components/Weekly.vue';
import Mine from './components/Mine.vue';

import useHandleNote from './hooks/useHandleNote';

const isCollapsed = ref(false);
const currentIndex = ref(0);
const compList = [Today, Weekly, Mine];
const containerRef = ref(null);
const showCollapseIcon = ref(true);

const currentComp = computed(() => compList[currentIndex.value]);

const { handleCreateNote } = useHandleNote();

const { apply, stop } = useMotion(containerRef, {
  initial: {
    opacity: 0,
    x: 400, // 初始从右侧偏移
  },
  enter: {
    opacity: 1,
    x: 0,
  },
  visible: {
    opacity: 1,
    x: 0,
    transition: {
      type: 'spring',
      stiffness: 250,
      damping: 20,
      mass: 0.5,
    },
  },
});

async function handleToggleCollapse() {
  showCollapseIcon.value = false;
  await nextTick();
  await invoke(isCollapsed.value ? 'expand_window' : 'collapse_window');
  isCollapsed.value = !isCollapsed.value;

  // 前置 否则apply会吞掉这个cb
  setTimeout(() => {
    showCollapseIcon.value = true;
  }, 1000);

  if (isCollapsed.value) {
    stop();
  } else {
    await nextTick();
    await apply('visible');
  }
}
</script>

<style lang="less" scoped>
::-webkit-scrollbar {
  display: none;
}

.gradient-text {
  --n-bezier: cubic-bezier(0.4, 0, 0.2, 1);
  --n-rotate: 252deg;
  --n-color-start: rgba(240, 160, 32, 0.6);
  --n-color-end: #f0a020;
  -webkit-background-clip: text;
  background-clip: text;
  color: #0000;
  white-space: nowrap;
  background-image: linear-gradient(
    var(--n-rotate),
    var(--n-color-start) 0%,
    var(--n-color-end) 100%
  );
  transition: --n-color-start 0.3s var(--n-bezier),
    --n-color-end 0.3s var(--n-bezier);
}

.toggle-wrapper {
  overflow: hidden;

  & :deep(.remixicon) {
    transform: scale(1.3);
    transform-origin: center;
  }
}
</style>
