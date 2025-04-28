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
const finished = ref(false);

const lastId = computed(() => noteList.value.at(-1)?.id || null);

async function handleCreateNote() {
    try {
        // 创建一定是空的
        const newNote = await invoke('create_note', {
            title: '',
            content: '',
        });
        // 创建之后 本地更新
        noteList.value.unshift(newNote);
        console.log('创建笔记成功', newNote);
    } catch (error) {
        console.log('创建笔记失败', error);
    }
}

async function handleGetTodayNotes() {
    try {
        const notes = await invoke("list_notes_by_id", {
            lastId: lastId.value,
            pageSize: 10,
        });
        console.log('获取笔记成功', notes);
        noteList.value = !lastId ? notes.data : noteList.value.concat(notes.data);
        finished.value = notes.data.length < 10;
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
        // 本地删除 不再重新获取
        const idx = noteList.value.findIndex(note => note.id === id);
        if (idx > -1) {
            noteList.value.splice(idx, 1);
        }
        console.log('更新笔记成功', notes);
    } catch (error) {
        console.log('更新笔记成功', error);
    }
}

export default function useHandleNote() {
    return {
        noteList,
        weeklyNoteList,
        finished,
        handleCreateNote,
        handleGetTodayNotes,
        handleGetWeeklyNotes,
        handleUpdateNote,
        handleMarkNote,
        handleDeleteNote,
    };
}
