<template>
  <EditorContent :editor="editor" />
</template>

<script setup>
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import Placeholder from '@tiptap/extension-placeholder';

const editor = useEditor({
  content: ``,
  extensions: [
    StarterKit,
    Highlight,
    Typography,
    Placeholder.configure({
      placeholder: 'Write something …',
    }),
  ],
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
