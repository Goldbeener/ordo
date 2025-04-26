<template>
  <bubble-menu
      v-if="editor"
      :editor="editor"
      :tippy-options="{ duration: 100 }"
  >
    <div class="bubble-menu">
      <button @click="editor.chain().focus().toggleBold().run()" :class="{ 'is-active': editor.isActive('bold') }">
        加粗
      </button>
      <button @click="handleToggleColor"
              :class="{ 'is-active': editor.isActive('textStyle', { color: '#ff6600' })}">
        标红
      </button>
      <button @click="editor.chain().focus().toggleStrike().run()" :class="{ 'is-active': editor.isActive('strike') }">
        中划线
      </button>
    </div>
  </bubble-menu>
  <EditorContent :editor="editor"/>
</template>

<script setup>
import {useEditor, EditorContent, BubbleMenu} from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Typography from '@tiptap/extension-typography';
import Placeholder from '@tiptap/extension-placeholder';
import TaskItem from '@tiptap/extension-task-item'
import TaskList from '@tiptap/extension-task-list'
import Paragraph from '@tiptap/extension-paragraph'
import {Color} from '@tiptap/extension-color'
import TextStyle from '@tiptap/extension-text-style'


const props = defineProps({
  content: {
    type: String,
  },
  editable: {
    type: Boolean,
    default: true
  },
  placeholder: {
    type: String,
    default: 'Write something …'
  }
});
const emit = defineEmits(['update']);
defineExpose({
  getEditorText
})

const editor = useEditor({
  editable: props.editable,
  content: props.content || '',
  extensions: [
    StarterKit,
    Paragraph,
    Highlight,
    Typography,
    Placeholder.configure({
      placeholder: props.placeholder,
    }),
    TaskList,
    TaskItem.configure({
      nested: true,
    }),
    TextStyle.configure({mergeNestedSpanStyles: true}),
    Color,
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

function getEditorText() {
  return editor.value.getText()
}

function handleToggleColor() {
  const color = '#ff6600';
  if (editor.value.isActive('textStyle', {color})) {
    editor.value.chain().focus().unsetColor().run()
  } else {
    editor.value.chain().focus().setColor(color).run()
  }
}
</script>

<style lang="less">
@import url('../styles/editor.less');

.prose :where(strong):not(:where([class~="not-prose"],[class~="not-prose"] *)) {
  color: inherit !important;
}

.bubble-menu {
  background-color: var(--white);
  border: 1px solid var(--gray-1);
  border-radius: 11.2px;
  box-shadow: var(--shadow);
  display: flex;
  padding: 3.2px;

  button {
    border-radius: 8px;
    color: var(--black) !important;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.15;
    padding: 6px 10px;
    transition: all .2s cubic-bezier(.65, .05, .36, 1);

    &:hover {
      background-color: var(--gray-3);
    }

    &.is-active {
      background-color: var(--theme);
    }
  }
}

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
