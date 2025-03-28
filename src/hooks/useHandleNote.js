import { invoke } from '@tauri-apps/api/core';

const noteList = ref([]);
async function handleCreateNote() {
  try {
    // 创建一定是空的
    const notes = await invoke('create_note', {
      title: '',
      content: '',
    });
    // 创建之后更新列表
    handleGetTodayNotes();
    console.log('创建笔记成功', notes);
  } catch (error) {
    console.log('创建笔记失败', error);
  }
}

async function handleGetTodayNotes() {
  try {
    const notes = await invoke('list_notes', {
      start_date: '2025-03-28 00:00:00',
      end_date: '2025-03-28 23:59:59',
    });
    console.log('获取笔记成功', notes);
    noteList.value = notes;
  } catch (error) {
    console.log('获取笔记失败', error);
  }
}

async function handleUpdateNote({ id, content }) {
  try {
    const notes = await invoke('update_note', {
      id,
      title: '',
      content,
    });
    console.log('更新笔记成功', notes);
  } catch (error) {
    console.log('更新笔记成功', error);
  }
}

async function handleDeleteNote(id) {
  try {
    const notes = await invoke('delete_note', { id });
    handleGetTodayNotes();
    console.log('更新笔记成功', notes);
  } catch (error) {
    console.log('更新笔记成功', error);
  }
}
export default function useHandleNote() {
  return {
    noteList,
    handleCreateNote,
    handleGetTodayNotes,
    handleUpdateNote,
    handleDeleteNote,
  };
}
