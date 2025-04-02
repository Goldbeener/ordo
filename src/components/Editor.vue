<template>
  <EditorContent :editor="editor" />
</template>

<script setup>
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import Placeholder from '@tiptap/extension-placeholder';
import TaskItem from '@tiptap/extension-task-item'
import TaskList from '@tiptap/extension-task-list'
import Paragraph from '@tiptap/extension-paragraph'

const props = defineProps({
  content: {
    type: String,
  },
});
const emit = defineEmits(['update']);

const editor = useEditor({
  content: props.content || '',
  extensions: [
    StarterKit,
    Paragraph,
    Highlight,
    Typography,
    Placeholder.configure({
      placeholder: 'Write something …',
    }),
    TaskList,
    TaskItem.configure({
      nested: true,
    }),
  ],
  editorProps: {
    attributes: {
      class:
        'prose prose-sm sm:prose-base lg:prose-lg xl:prose-2xl mb-5 focus:outline-none',
    },
  },
  onUpdate() {
    emit('update', editor.value.getHTML());
  },
});
</script>

<style lang="less">
@import url('../styles/editor.less');

.tiptap {
  margin-left: 0;
  p.is-editor-empty:first-child::before {
    color: #adb5bd;
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
  }
  ul[data-type="taskList"] {
    margin: 0;
    padding-left: 0;
    li {
      display: flex;
      align-items: baseline;
      padding-inline-start: 0;
      label {
        margin-right: 14px;
        font-size: 16px;
      }
    }
  }
}
</style>
