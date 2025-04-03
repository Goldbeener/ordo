<template>
<div class="container w-screen flex flex-col items-center py-[32px] px-[16px]">
  <n-avatar
    round
    :size="66"
    src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
    class="mb-6"
  />
  <n-card hoverable>设置定时任务</n-card>
</div>
</template>

<script setup>

</script>

<style scoped lang="less">
.container {
  & :deep(.n-card) {
    --n-border-radius: 10px !important;
    --n-padding-bottom: 14px !important;
  }
}
</style>