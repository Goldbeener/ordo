<template>
  <div class="h-full w-full flex px-4 py-3 overflow-scroll">
    <n-timeline>
      <n-timeline-item v-for="(dayNotes, key) in timeLineData" :key="key">
        <n-collapse :default-expanded-names="['1']">
          <n-collapse-item  :title="key" name="1">
            <Editor v-for="note in dayNotes" :content="note.content" :editable="false"/>
          </n-collapse-item>
        </n-collapse>
      </n-timeline-item>
    </n-timeline>
  </div>
</template>

<script setup>
import { parseISO, getDay, format } from 'date-fns'

import Editor from "./Editor.vue";

import useHandleNote from "../hooks/useHandleNote.js";

const { weeklyNoteList, handleGetWeeklyNotes } = useHandleNote();
const weekDays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六']

// 格式化数据 按天聚合
const timeLineData = computed(() => {
  const result = {};
  weeklyNoteList.value
      .filter(note => note.content.trim())
      .forEach(note => {
        const weekday = weekDays[getDay(parseISO(note.create_time))];
        if(!result[weekday]) {
          result[weekday] = [note]
        } else {
          result[weekday].push(note)
        }
      })
  return result
})

handleGetWeeklyNotes().then(() => {
  console.log('本周数据', weeklyNoteList.value)
})

function formatTime(dateString) {
  return format(parseISO(dateString), 'HH:mm')
}

</script>

<style scoped lang="less">
:deep(.n-timeline) {
  &-item:nth-child(1) { background: #F2F5F9; } /* 周一 */
  &-item:nth-child(2) { background: #E3EDF7; } /* 周二 */
  &-item:nth-child(3) { background: #E1F3F1; } /* 周三 */
  &-item:nth-child(4) { background: #E9F6E5; } /* 周四 */
  &-item:nth-child(5) { background: #FCF7DC; } /* 周五 */
  &-item:nth-child(6) { background: #FDEEDC; } /* 周六 */
  &-item:nth-child(7) { background: #FCE8E9; } /* 周日 */

  .n-timeline-item {
    padding-top: 12px;
  }
}
</style>