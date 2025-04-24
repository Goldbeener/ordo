<template>
  <n-card style="width: 80vw">
    <n-form ref="formRef" :model="model">
      <n-form-item path="name" class="w-[60vw]" label="任务名称">
        <n-input v-model:value="model.name" placeholder="主要是干什么" @keydown.enter.prevent/>
      </n-form-item>
      <n-form-item path="desc" label="任务描述" class="w-[60vw]">
        <n-input v-model:value="model.desc" placeholder="有什么额外关注" @keydown.enter.prevent/>
      </n-form-item>
      <n-form-item path="datetime" label="任务时间">
        <n-date-picker v-model:value="model.datetime" placeholder="什么时间提醒" type="datetime"/>
      </n-form-item>
      <n-form-item path="repeatType" label="重复类型">
        <n-select
            class="w-[50vw]"
            v-model:value="model.repeatType"
            placeholder="是否重复"
            :options="repeatOptions"
        />
      </n-form-item>
      <n-row :gutter="[0, 24]">
        <n-col :span="24">
          <div class="flex justify-end">
            <n-button round class="mr-4" @click="emit('finished')">取消</n-button>
            <n-button
                :disabled="model.name === null"
                round
                type="primary"
                @click="handleAddSchedule"
            >
              创建任务
            </n-button>
          </div>
        </n-col>
      </n-row>
    </n-form>
  </n-card>
</template>

<script setup>
import {invoke} from "@tauri-apps/api/core";

const emit = defineEmits(["finished"]);

const formRef = ref(null)
const model = ref({
  name: null,
  desc: null,
  datetime: null,
  repeatType: null
})
const repeatValue = {
  '不重复': '',
  '每周': 'weekly',
  '每月': 'monthly',
}
const repeatOptions = ['不重复', '每周', '每月'].map(
    v => ({
      label: v,
      value: repeatValue[v]
    })
)

async function handleAddSchedule() {

  const localDate = new Date(model.value.datetime);
  const isoString = localDate.toISOString();

  await invoke('add_schedule', {
    name: model.value.name,
    description: model.value.desc,
    datetime: isoString,
    repeatType: model.value.repeatType || null
  });
  emit('finished', true);
}
</script>

<style scoped lang="less">

</style>