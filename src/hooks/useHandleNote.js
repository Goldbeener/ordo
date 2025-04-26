import {invoke} from '@tauri-apps/api/core';
import {
    startOfDay,
    endOfDay,
    subDays,
    startOfWeek,
    endOfWeek,
    setHours,
    setMinutes,
    setSeconds,
    setMilliseconds
} from 'date-fns'

const noteList = ref([]);
const weeklyNoteList = ref([]);

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
        const notes = await invoke("list_notes", {
            page: 1,
            pageSize: 10,
        });
        console.log('获取笔记成功', notes);
        noteList.value = notes.data;
    } catch (error) {
        console.log('获取笔记失败', error);
    }
}

async function handleGetWeeklyNotes() {
    try {
        const today = new Date();
        const mondayStart = setMilliseconds(
            setSeconds(setMinutes(setHours(startOfWeek(today, {weekStartsOn: 1}), 0), 0), 0),
            0
        )
        const sundayEnd = setMilliseconds(
            setSeconds(setMinutes(setHours(endOfWeek(today, {weekStartsOn: 1}), 23), 59), 59),
            999
        )
        weeklyNoteList.value = await invoke("list_notes_by_date", {
            startDate: mondayStart.toISOString(),
            endDate: sundayEnd.toISOString()
        });
    } catch (error) {
        console.log('获取笔记失败', error);
    }
}

async function handleUpdateNote({id, content}) {
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

async function handleMarkNote(note) {
    try {
        const notes = await invoke('update_note', note);
        console.log('更新笔记成功', notes);
    } catch (error) {
        console.log('更新笔记成功', error);
    }
}

async function handleDeleteNote(id) {
    try {
        const notes = await invoke('delete_note', {id});
        handleGetTodayNotes();
        console.log('更新笔记成功', notes);
    } catch (error) {
        console.log('更新笔记成功', error);
    }
}

export default function useHandleNote() {
    return {
        noteList,
        weeklyNoteList,
        handleCreateNote,
        handleGetTodayNotes,
        handleGetWeeklyNotes,
        handleUpdateNote,
        handleMarkNote,
        handleDeleteNote,
    };
}
