<template>
  <div class="container w-screen flex flex-col items-center py-[32px] px-[16px]">
    <div v-html="svg" class="mb-6" @click="generateAvatar()"></div>
    <n-collapse class="bg-white" default-expanded-names="1" accordion>
      <n-collapse-item title="设置定时任务" name="1">
        <template #arrow>
          <RiTimerLine/>
        </template>
        <div>
          <n-card>
            <n-form ref="formRef" :model="model">
              <n-form-item path="name" label="任务名称">
                <n-input v-model:value="model.name" @keydown.enter.prevent/>
              </n-form-item>
              <n-form-item path="desc" label="任务描述">
                <n-input v-model:value="model.desc" @keydown.enter.prevent/>
              </n-form-item>
              <n-form-item path="datetime" label="任务时间">
                <n-date-picker v-model:value="model.datetime" type="datetime"/>
              </n-form-item>
              <n-form-item path="repeatType" label="重复类型">
                <n-select
                    v-model:value="model.repeatType"
                    placeholder="是否重复"
                    :options="repeatOptions"
                />
              </n-form-item>
              <n-row :gutter="[0, 24]">
                <n-col :span="24">
                  <div class="flex justify-end">
                    <n-button
                        :disabled="model.name === null"
                        round
                        type="primary"
                        @click="handleValidateButtonClick"
                    >
                      创建任务
                    </n-button>
                  </div>
                </n-col>
              </n-row>
            </n-form>
          </n-card>
        </div>
      </n-collapse-item>
      <n-collapse-item title="静态类型" name="2">
        <div>Java</div>
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<script setup>
import {invoke} from '@tauri-apps/api/core';
import {createAvatar} from '@dicebear/core'
import {funEmoji} from '@dicebear/collection'
import {format, parseISO, addMinutes} from 'date-fns';
import {RiTimerLine} from '@remixicon/vue'

const svg = ref('')
const newSchedule = ref({
  name: '',
  datetime: '',
  repeatType: ''
});
const formRef = ref(null)
const rPasswordFormItemRef = ref(null)
const model = ref({
  name: null,
  password: null,
  reenteredPassword: null
})
const repeatOptions = ['', 'veli good', 'emazing', 'lidiculous'].map(
    v => ({
      label: v,
      value: v
    })
)

function validatePasswordStartWith(rule, value) {
  return (
      !!modelRef.value.password
      && modelRef.value.password.startsWith(value)
      && modelRef.value.password.length >= value.length
  )
}

function validatePasswordSame(rule, value) {
  return value === modelRef.value.password
}


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

function handleAddSchedule() {
  newSchedule.value = {
    name: 'test',
    datetime: addMinutes(Date.now(), '1'),
    repeatType: 'weekly'
  }

  console.log('?????', newSchedule.value);

  const localDate = new Date(newSchedule.value.datetime);
  const isoString = localDate.toISOString();

  invoke('add_schedule', {
    name: newSchedule.value.name,
    datetime: isoString,
    repeatType: newSchedule.value.repeatType || null
  });
}

</script>

<style scoped lang="less">
.container {
  & :deep(.n-collapse) {
    border-radius: 12px;

    .n-collapse-item__header-main {
      padding: 16px 8px;
      font-weight: bold;
    }

    .n-collapse-item {
      .n-collapse-item-arrow {
        transform: rotate(0) !important;
      }

      &__content-inner {
        padding: 16px !important;
      }
    }

    .n-form-item {
      width: 75%
    }
  }
}
</style>