// src/commands.rs
use crate::db::{Note, NoteDatabase};
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use tauri::{command, State};

#[command]
pub async fn create_note(
    db: State<'_, Mutex<NoteDatabase>>,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
) -> Result<i64, String> {
    let note = Note {
        id: None,
        title,
        content,
        create_time: Utc::now(),
        last_modify_time: Utc::now(),
        tags,
    };

    db.lock()
        .map_err(|_| "Failed to acquire database lock".to_string())?
        .create_note(&note)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_note(
    db: State<'_, Mutex<NoteDatabase>>,
    id: i64,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
) -> Result<(), String> {
    let note = Note {
        id: Some(id),
        title,
        content,
        create_time: Utc::now(), // 不会更新创建时间
        last_modify_time: Utc::now(),
        tags,
    };

    db.lock()
        .map_err(|_| "Failed to acquire database lock".to_string())?
        .update_note(&note)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_note(db: State<'_, Mutex<NoteDatabase>>, id: i64) -> Result<(), String> {
    db.lock()
        .map_err(|_| "Failed to acquire database lock".to_string())?
        .delete_note(id)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_note(db: State<'_, Mutex<NoteDatabase>>, id: i64) -> Result<Option<Note>, String> {
    db.lock()
        .map_err(|_| "Failed to acquire database lock".to_string())?
        .get_note_by_id(id)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn list_notes(
    db: State<'_, Mutex<NoteDatabase>>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Vec<Note>, String> {
    println!("Received start_date: {:?}", start_date);
    println!("Received end_date: {:?}", end_date);

    let start = start_date.and_then(|d| {
        DateTime::parse_from_rfc3339(&d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });
    let end = end_date.and_then(|d| {
        DateTime::parse_from_rfc3339(&d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    db.lock()
        .map_err(|_| "Failed to acquire database lock".to_string())?
        .list_notes(start, end)
        .map_err(|e| e.to_string())
}
