<template>
  <div class="h-full w-full flex py-8 overflow-scroll">
    <n-timeline>
      <n-timeline-item v-for="(dayNotes, key) in timeLineData" :key="key">
        <n-collapse :default-expanded-names="['1']">
          <n-collapse-item :title="key" name="1">
            <Editor
              v-for="note in dayNotes"
              :content="note.content"
              :editable="false"
            />
          </n-collapse-item>
        </n-collapse>
      </n-timeline-item>
    </n-timeline>
  </div>
</template>

<script setup>
import { parseISO, getDay, format } from 'date-fns';

import Editor from './Editor.vue';

import useHandleNote from '../hooks/useHandleNote.js';

const { weeklyNoteList, handleGetWeeklyNotes } = useHandleNote();
const weekDays = [
  '星期日',
  '星期一',
  '星期二',
  '星期三',
  '星期四',
  '星期五',
  '星期六',
];

// 格式化数据 按天聚合
const timeLineData = computed(() => {
  const result = {};
  weeklyNoteList.value
    .filter((note) => note.content.trim())
    .forEach((note) => {
      const weekday = weekDays[getDay(parseISO(note.create_time))];
      if (!result[weekday]) {
        result[weekday] = [note];
      } else {
        result[weekday].push(note);
      }
    });
  return result;
});

handleGetWeeklyNotes().then(() => {
  console.log('本周数据', weeklyNoteList.value);
});

function formatTime(dateString) {
  return format(parseISO(dateString), 'HH:mm');
}
</script>

<style scoped lang="less">
:deep(.n-timeline) {
  &-item {
    padding-top: 0px !important;
  }

  &-item:first-of-type {
    padding-top: 20px !important;
  }

  &-item:first-of-type {
    border-top-left-radius: 12px;
    border-top-right-radius: 12px;
  }

  &-item:last-of-type {
    border-bottom-left-radius: 12px;
    border-bottom-right-radius: 12px;
  }

  &-item:nth-child(1) {
    background: #f2f5f9;

    .n-timeline-item-timeline__circle {
      border-color: #1f2d3d !important;
      margin-top: 18px !important;
    }
  }

  /* 周一 */

  &-item:nth-child(2) {
    background: #e3edf7;

    .n-timeline-item-timeline__circle {
      border-color: #1a293b !important;
    }
  }

  /* 周二 */

  &-item:nth-child(3) {
    background: #e1f3f1;

    .n-timeline-item-timeline__circle {
      border-color: #1e3a36 !important;
    }
  }

  /* 周三 */

  &-item:nth-child(4) {
    background: #e9f6e5;

    .n-timeline-item-timeline__circle {
      border-color: #2c3e22 !important;
    }
  }

  /* 周四 */

  &-item:nth-child(5) {
    background: #fcf7dc;

    .n-timeline-item-timeline__circle {
      border-color: #5a4e1e !important;
    }
  }

  /* 周五 */

  &-item:nth-child(6) {
    background: #fdeedc;

    .n-timeline-item-timeline__circle {
      border-color: #5c3a1e !important;
    }
  }

  /* 周六 */

  &-item:nth-child(7) {
    background: #fce8e9;

    .n-timeline-item-timeline__circle {
      border-color: #5a1f2d !important;
    }
  }

  .n-timeline-item {
    padding-top: 12px;

    &:first-of-type {
      .n-timeline-item-timeline__line {
        top: 34px !important;
      }
    }

    &-timeline__line {
      left: 12px !important;
    }

    &-timeline__circle {
      margin: 0 auto;
      width: 16px !important;
      height: 16px !important;

      &::after {
        display: block;
        content: ' ';
        width: 2px;
      }
    }
  }

  .n-collapse {
    --n-title-font-weight: 700 !important;

    .n-collapse-item-arrow {
      display: none;
    }

    .n-collapse-item__content-inner {
      padding-top: 0;
    }
  }
}
</style>
