<template>
  <EditorContent :editor="editor" />
</template>

<script setup>
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import Placeholder from '@tiptap/extension-placeholder';

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
    Highlight,
    Typography,
    Placeholder.configure({
      placeholder: 'Write something …',
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
}
</style>
