use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub create_time: DateTime<Utc>,
    pub last_modify_time: DateTime<Utc>,
    pub tags: Option<Vec<String>>,
}

pub struct NoteDatabase {
    conn: Connection,
}

impl NoteDatabase {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                create_time DATETIME NOT NULL,
                last_modify_time DATETIME NOT NULL,
                tags TEXT
            )",
            [],
        )?;
        Ok(NoteDatabase { conn })
    }

    pub fn create_note(&self, note: &Note) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO notes (title, content, create_time, last_modify_time, tags) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                note.title,
                note.content,
                note.create_time.to_rfc3339(),
                note.last_modify_time.to_rfc3339(),
                note.tags.as_ref().map(|t| t.join(","))
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_note(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "UPDATE notes 
            SET title = ?1, content = ?2, last_modify_time = ?3, tags = ?4 
            WHERE id = ?5",
            params![
                note.title,
                note.content,
                Utc::now().to_rfc3339(),
                note.tags.as_ref().map(|t| t.join(",")),
                note.id
            ],
        )?;
        Ok(())
    }

    pub fn delete_note(&self, note_id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM notes WHERE id = ?1", params![note_id])?;
        Ok(())
    }

    pub fn get_note_by_id(&self, note_id: i64) -> Result<Option<Note>> {
        let mut stmt = self.conn.prepare("SELECT * FROM notes WHERE id = ?1")?;
        let note_iter = stmt.query_map(params![note_id], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                create_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modify_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                tags: row
                    .get::<_, Option<String>>(5)?
                    .map(|t| t.split(',').map(String::from).collect()),
            })
        })?;

        let mut notes: Vec<Note> = note_iter.collect::<Result<Vec<Note>>>()?;
        Ok(notes.pop())
    }

    pub fn list_notes_by_date(
        &self,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Note>> {
        let mut query = "SELECT * FROM notes".to_string();
        let mut conditions = Vec::new();
    
        if let Some(start) = start_date {
            conditions.push(format!("create_time >= '{}'", start.to_rfc3339()));
        }
    
        if let Some(end) = end_date {
            conditions.push(format!("create_time <= '{}'", end.to_rfc3339()));
        }
    
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }
    
        query.push_str(" ORDER BY create_time DESC");
    
        let mut stmt = self.conn.prepare(&query)?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                create_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modify_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                tags: row
                    .get::<_, Option<String>>(5)?
                    .map(|t| t.split(',').map(String::from).collect()),
            })
        })?;
    
        let notes: Vec<Note> = note_iter.collect::<Result<Vec<Note>>>()?;
        Ok(notes)
    }

    pub fn list_notes(&self, page: usize, page_size: usize) -> Result<(Vec<Note>, usize)> {
        // 先获取总记录数
        let count_query = "SELECT COUNT(*) FROM notes";
        let total_count: usize = self.conn.query_row(count_query, [], |row| row.get(0))?;

        // 计算分页参数
        let offset = (page - 1) * page_size;

        // 构建查询
        let query = format!(
            "SELECT * FROM notes ORDER BY create_time DESC LIMIT {} OFFSET {}",
            page_size, offset
        );

        let mut stmt = self.conn.prepare(&query)?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                create_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modify_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                tags: row
                    .get::<_, Option<String>>(5)?
                    .map(|t| t.split(',').map(String::from).collect()),
            })
        })?;

        let notes: Vec<Note> = note_iter.collect::<Result<Vec<Note>>>()?;

        // 返回笔记列表和总记录数
        Ok((notes, total_count))
    }

    // 查询带tag的notes
    pub fn list_tag_notes(&self, page: usize, page_size: usize) -> Result<(Vec<Note>, usize)> {
        // 先获取总记录数
        let count_query = "SELECT COUNT(*) FROM notes WHERE tags IS NOT NULL AND tags != ''";
        let total_count: usize = self.conn.query_row(count_query, [], |row| row.get(0))?;

        // 计算分页参数
        let offset = (page - 1) * page_size;

        // 构建查询
        let query = format!(
            "SELECT * FROM notes WHERE tags IS NOT NULL AND tags != '' 
             ORDER BY create_time DESC LIMIT {} OFFSET {}",
            page_size, offset
        );

        let mut stmt = self.conn.prepare(&query)?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                create_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&Utc),
                last_modify_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                tags: row
                    .get::<_, Option<String>>(5)?
                    .map(|t| t.split(',').map(String::from).collect()),
            })
        })?;

        let notes: Vec<Note> = note_iter.collect::<Result<Vec<Note>>>()?;

        // 返回笔记列表和总记录数
        Ok((notes, total_count))
    }
}
