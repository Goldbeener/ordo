use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub id: Option<i64>,
    pub name: String,
    pub datetime: String,
    pub description: Option<String>,
    pub repeat_type: Option<String>, // "weekly", "monthly", or null for non-recurring
    pub created_at: String,
    pub updated_at: String,
}

pub struct DbTimer {
    conn: Connection,
}

impl DbTimer {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // 创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schedules (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                datetime TEXT NOT NULL,
                repeat_type TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_schedule(&self, schedule: &Schedule) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO schedules (name, description, datetime, repeat_type, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                schedule.name,
                schedule.description,
                schedule.datetime,
                schedule.repeat_type,
                now,
                now
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_all_schedules(&self) -> Result<Vec<Schedule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, datetime, repeat_type, created_at, updated_at
            FROM schedules
            ORDER BY created_at DESC",
        )?;

        let schedule_iter = stmt.query_map([], |row| {
            Ok(Schedule {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                datetime: row.get(3)?,
                repeat_type: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut schedules = Vec::new();
        for schedule in schedule_iter {
            schedules.push(schedule?);
        }

        Ok(schedules)
    }

    pub fn get_schedule_by_id(&self, id: i64) -> Result<Option<Schedule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, datetime, repeat_type, created_at, updated_at
            FROM schedules
            WHERE id = ?1",
        )?;

        let schedule = stmt.query_row(params![id], |row| {
            Ok(Schedule {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                datetime: row.get(3)?,
                repeat_type: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        });

        match schedule {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn update_schedule(&self, schedule: &Schedule) -> Result<usize> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE schedules
            SET name = ?1, description = ?2, datetime = ?3, repeat_type = ?4, updated_at = ?5
            WHERE id = ?6",
            params![
                schedule.name,
                schedule.description,
                schedule.datetime,
                schedule.repeat_type,
                now,
                schedule.id,
            ],
        )
    }

    pub fn delete_schedule(&self, id: i64) -> Result<usize> {
        self.conn
            .execute("DELETE FROM schedules WHERE id = ?1", params![id])
    }
}
