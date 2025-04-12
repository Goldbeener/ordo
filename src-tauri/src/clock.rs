use std::sync::Arc;
use std::time::Duration;
use tauri::{Manager, State};
use rusqlite::{Connection, params};
use chrono::{Local, NaiveDateTime};
use tokio::sync::Mutex;
use tauri_plugin_notification::NotificationExt;

#[derive(Debug)]
struct Reminder {
    id: i64,
    title: String,
    time: NaiveDateTime,
    repeat: String,
}

struct Db(pub Arc<Mutex<Connection>>);

#[tauri::command]
pub async fn add_reminder(
    db: State<'_, Db>,
    title: String,
    time: String,
    repeat: String,
) -> Result<(), String> {
    let conn = db.0.lock().await;
    let time = NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M") // adjust if needed
        .map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO reminders (title, time, repeat) VALUES (?1, ?2, ?3)",
        params![title, time, repeat],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn check_reminders(app: tauri::AppHandle, db: Arc<Mutex<Connection>>) {
    tauri::async_runtime::spawn(async move {
        loop {
            let now = Local::now().naive_local();
            let conn = db.lock().await;

            let mut stmt = conn.prepare(
                "SELECT id, title, time, repeat FROM reminders"
            ).unwrap();

            let reminders = stmt.query_map([], |row| {
                Ok(Reminder {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    time: row.get(2)?,
                    repeat: row.get(3)?,
                })
            }).unwrap();

            for reminder in reminders {
                let reminder = reminder.unwrap();
                if (reminder.time - now).num_seconds().abs() < 60 {
                    let title = reminder.title.clone();

                    app.notification()
                        .builder()
                        .title("提醒事项")
                        .body(&title)
                        .show()
                        .unwrap();

                    // 如果不重复，删除
                    if reminder.repeat == "none" {
                        conn.execute("DELETE FROM reminders WHERE id = ?", params![reminder.id]).unwrap();
                    } else {
                        // 如果是每日/每周，更新时间
                        let next_time = match reminder.repeat.as_str() {
                            "daily" => reminder.time + chrono::Duration::days(1),
                            "weekly" => reminder.time + chrono::Duration::weeks(1),
                            _ => reminder.time,
                        };
                        conn.execute("UPDATE reminders SET time = ? WHERE id = ?", params![next_time, reminder.id]).unwrap();
                    }
                }
            }

            drop(conn);
            tokio::time::sleep(Duration::from_secs(30)).await;
        }

    });
}

pub fn init_db() -> Arc<Mutex<Connection>> {
    let conn = Connection::open("reminders.db").expect("数据库打开失败");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            time TEXT NOT NULL,
            repeat TEXT NOT NULL
        )", [],
    ).unwrap();
    Arc::new(Mutex::new(conn))
}