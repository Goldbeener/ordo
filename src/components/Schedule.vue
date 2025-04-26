<template>
  <div class="schedule-list flex flex-col items-center w-full p-2">
    <!--  已定时任务-->
    <div class="w-full grid grid-cols-6 py-1" v-for="schedule in scheduleList" :key="schedule.id">
      <div class="col-span-2 flex flex-col justify-center">
        <div class="text-base font-bold">{{ schedule.name }}</div>
        <div class="text-sm">{{ schedule.description }}</div>
      </div>
      <div class="col-span-3 flex flex-col justify-center">
        <div class="text-sm">{{ formatRepeat(schedule) }}</div>
        <div class="text-sm">{{ formatDate(schedule) }}</div>
      </div>
      <div class="col-span-1 flex flex-col justify-center items-end">
        <n-dropdown trigger="click" :options="options" @select="param => handleSelect(schedule, param)">
          <div class="w-9 h-9 rounded-lg hover:bg-gray-200 flex items-center justify-center">
            <RiMore2Fill/>
          </div>
        </n-dropdown>
      </div>

    </div>
  </div>
</template>

<script setup>
import {invoke} from '@tauri-apps/api/core';
import {format, getDay, getDate} from 'date-fns';
import {RiMore2Fill} from '@remixicon/vue'

defineExpose({loadSchedule})

const scheduleList = ref([]);
const repeatOptionMap = {
  'weekly': '每周',
  'monthly': '每月'
}
const options = [
  {
    label: '删除',
    key: 'delete'
  },
  // {
  //   label: '编辑',
  //   key: 'edit'
  // },
]
const weekDays = ['一', '二', '三', '四', '五', '六', '日']

loadSchedule()

async function loadSchedule() {
  const data = await invoke('get_schedules', {})
  console.log('???已设置的提醒事项', data);
  scheduleList.value = data;
}

function formatRepeat(schedule) {
  const {datetime, repeat_type} = schedule;
  const date = new Date(datetime);
  if (repeat_type === 'weekly') {
    return repeatOptionMap[repeat_type] + weekDays[getDay(date) - 1];
  } else if (repeat_type === 'monthly') {
    return repeatOptionMap[repeat_type] + getDate(date) + '号';
  }
  return '不重复'
}

function formatDate(schedule) {
  const {datetime, repeat_type} = schedule;
  let formatStr = 'HH:mm:ss';
  if (!repeat_type) {
    formatStr = 'yyyy-MM-dd HH:mm:ss';
  }
  return format(new Date(datetime), formatStr)
}

async function handleSelect(schedule, param) {
  if (param === 'delete') {
    await deleteSchedule(schedule.id);
    loadSchedule()
  } else if (param === 'edit') {
    console.log('编辑', schedule);
  }
}

function deleteSchedule(id) {
  invoke('delete_schedule', {id})
}
</script>

<style scoped lang="less">

</style>