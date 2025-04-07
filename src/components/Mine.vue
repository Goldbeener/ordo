<template>
<div class="container w-screen flex flex-col items-center py-[32px] px-[16px]">
  <div v-html="svg" class="mb-6" @click="generateAvatar()"></div>
  <n-card hoverable>设置定时任务</n-card>
</div>
</template>

<script setup>
import { createAvatar } from '@dicebear/core'
import { funEmoji } from '@dicebear/collection'

const svg = ref('')
let seed = localStorage.getItem('avatar-seed')

generateAvatar(seed);

function generateAvatar(seed) {
  if (!seed) {
    seed = Math.random().toString(36).substring(2, 10)
    localStorage.setItem('avatar-seed', seed)
  }// 可以换其他风格

  // 生成 SVG 头像
  svg.value = createAvatar(funEmoji, {
    seed,
    size: 96,
    radius: 50
  }).toString()
}

</script>

<style scoped lang="less">
.container {
  & :deep(.n-card) {
    --n-border-radius: 10px !important;
    --n-padding-bottom: 14px !important;
  }
}
</style>