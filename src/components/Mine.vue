<template>
  <div
    v-bind="$attrs"
    class="container w-screen flex flex-col items-center py-[32px] px-[16px]"
  >
    <div class="w-full rounded-lg p-5 flex items-center mb-6 bg-white">
      <n-tooltip trigger="hover" placement="right-start">
        <template #trigger>
          <div v-html="svg" class="mr-5" @click="generateAvatar()"></div>
        </template>
        点击切换你的心情
      </n-tooltip>
      <div class="flex items-center flex-1">
        <div class="p-2 flex flex-col items-center mr-4">
          <n-number-animation :from="0" :to="totalNotesCount" />
          <div>笔记</div>
        </div>
        <div class="p-2 flex flex-col items-center">
          <n-number-animation :from="0" :to="totalTaggedNotesCount" />
          <div>点子</div>
        </div>
      </div>
    </div>

    <n-collapse
      class="bg-white"
      default-expanded-names="1"
      accordion
      :trigger-areas="['main']"
    >
      <n-collapse-item title="定时任务" name="1">
        <template #header-extra>
          <div class="cursor-pointer" @click="showModal = true">
            <RiSettings3Line />
          </div>
        </template>
        <template #arrow>
          <RiTimerLine />
        </template>
        <div>
          <Schedule ref="scheduleRef"></Schedule>
        </div>
      </n-collapse-item>
      <n-collapse-item title="番茄时钟" name="2">
        <template #arrow>
          <RiTimerFlashLine />
        </template>
        <div>开启番茄时钟</div>
      </n-collapse-item>
    </n-collapse>
  </div>
  <n-modal v-model:show="showModal">
    <CreateSchedue @finished="handleAddFinished" />
  </n-modal>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { createAvatar } from '@dicebear/core';
import { funEmoji } from '@dicebear/collection';
import { RiTimerLine, RiSettings3Line, RiTimerFlashLine } from '@remixicon/vue';
import Schedule from './Schedule.vue';
import CreateSchedue from './CreateSchedue.vue';

const scheduleRef = ref(null);
const svg = ref('');
const showModal = ref(false);
const totalNotesCount = ref(0);
const totalTaggedNotesCount = ref(0);
let seed = localStorage.getItem('avatar-seed');

generateAvatar(seed);
getNotesCount();

async function getNotesCount() {
  const data = await invoke('get_notes_count');
  totalNotesCount.value = data[0];
  totalTaggedNotesCount.value = data[1];
}

function generateAvatar(seed) {
  if (!seed) {
    seed = Math.random().toString(36).substring(2, 10);
    localStorage.setItem('avatar-seed', seed);
  } // 可以换其他风格

  // 生成 SVG 头像
  svg.value = createAvatar(funEmoji, {
    seed,
    size: 90,
    radius: 50,
  }).toString();
}

function handleAddFinished(needRefresh) {
  console.log('是否需要刷新', needRefresh, scheduleRef.value);
  showModal.value = false;
  if (needRefresh) {
    scheduleRef.value?.loadSchedule();
  }
}
</script>

<style scoped lang="less">
.container {
  & :deep(.n-collapse) {
    border-radius: 12px;

    .n-collapse-item__header {
      padding-top: 0;
    }

    .n-collapse-item__header-main {
      padding: 16px 8px;
      font-weight: bold;
    }

    .n-collapse-item__header-extra {
      padding-right: 8px;
    }

    .n-collapse-item {
      margin-top: 0;

      .n-collapse-item-arrow {
        transform: rotate(0) !important;
      }

      &__content-inner {
        padding: 16px !important;
      }
    }

    .n-form-item {
      width: 75%;
    }
  }
}
</style>
