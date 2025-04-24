use crate::db_timer::{DbTimer, Schedule};
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration as StdDuration;
use tauri::State;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use tokio::time;

// 定义一个全局状态，存储数据库连接和定时器任务
// 定义一个全局状态，存储数据库连接和定时器任务
pub struct SchedulerState {
    pub db_timer: Arc<Mutex<DbTimer>>,
    // 存储任务ID和对应的handle，用于取消任务
    pub tasks: Arc<Mutex<HashMap<i64, tokio::task::JoinHandle<()>>>>,
}

// 确保SchedulerState可以安全地在线程间传递
unsafe impl Send for SchedulerState {}
unsafe impl Sync for SchedulerState {}

// 初始化数据库并创建全局状态
pub fn init_timer_db(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir().expect("无法获取应用数据目录");
    println!("应用数据目录{}", app_dir.display());
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("timers.db");
    let db_timer = DbTimer::new(&db_path)?;

    // 创建全局状态
    let scheduler_state = SchedulerState {
        db_timer: Arc::new(Mutex::new(db_timer)),
        tasks: Arc::new(Mutex::new(HashMap::new())),
    };

    // 将状态注册到app
    app.manage(scheduler_state);

    // 启动时加载所有定时任务
    let app_handle = app.app_handle();
    start_all_schedules(&app_handle)?;

    Ok(())
}

// 添加定时任务的命令
#[tauri::command]
pub async fn add_schedule(
    state: State<'_, SchedulerState>,
    app_handle: AppHandle,
    name: String,
    datetime: String,
    description: Option<String>,
    repeat_type: Option<String>,
) -> Result<i64, String> {
    // 创建一个新的作用域来限制锁的持有时间
    let id = {
        let schedule = Schedule {
            id: None,
            name: name.clone(),
            description: description.clone(),
            datetime: datetime.clone(),
            repeat_type: repeat_type.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        // 获取锁并在这个作用域内使用它
        let db = state.db_timer.lock().map_err(|e| e.to_string())?;
        db.add_schedule(&schedule).map_err(|e| e.to_string())?
    };

    println!("收到定时任务{}", name.clone());

    // 添加完任务后，启动这个任务
    let schedule_with_id = Schedule {
        id: Some(id),
        name,
        description,
        datetime,
        repeat_type,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    start_schedule(&app_handle, schedule_with_id, state.tasks.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(id)
}

// 获取所有定时任务的命令
#[tauri::command]
pub fn get_schedules(state: State<'_, SchedulerState>) -> Result<Vec<Schedule>, String> {
    let db = state.db_timer.lock().map_err(|e| e.to_string())?;
    db.get_all_schedules().map_err(|e| e.to_string())
}

// 删除定时任务的命令
#[tauri::command]
pub fn delete_schedule(state: State<'_, SchedulerState>, id: i64) -> Result<(), String> {
    // 先取消正在运行的任务
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    if let Some(handle) = tasks.remove(&id) {
        handle.abort();
    }

    // 从数据库中删除
    let db = state.db_timer.lock().map_err(|e| e.to_string())?;
    db.delete_schedule(id).map_err(|e| e.to_string())?;

    Ok(())
}

// 启动所有定时任务
pub fn start_all_schedules(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let state = app_handle.state::<SchedulerState>();
    let db = state.db_timer.lock().unwrap();
    let schedules = db.get_all_schedules()?;
    drop(db); // 释放锁

    // 启动所有任务
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        for schedule in schedules {
            if let Err(e) = start_schedule(app_handle, schedule, state.tasks.clone()).await {
                eprintln!("启动任务失败: {}", e);
            }
        }
    });

    Ok(())
}

// 启动单个定时任务
async fn start_schedule(
    app_handle: &AppHandle,
    schedule: Schedule,
    tasks: Arc<Mutex<HashMap<i64, tokio::task::JoinHandle<()>>>>,
) -> Result<(), String> {
    // 改为返回 Result<(), String>
    let schedule_id = schedule.id.unwrap();
    let schedule_name = schedule.name.clone();
    let schedule_description = schedule.description.clone();
    let schedule_datetime = schedule.datetime.clone();
    let repeat_type = schedule.repeat_type.clone();

    // 解析时间
    let target_time = DateTime::parse_from_rfc3339(&schedule_datetime)
        .map_err(|e| format!("无法解析时间: {}", e))?
        .with_timezone(&Utc);

    let app_handle_clone = app_handle.clone();

    // 创建异步任务
    let handle = tokio::spawn(async move {
        loop {
            let now = Utc::now();
            if now < target_time {
                // 计算需要等待的时间
                let wait_duration = target_time.signed_duration_since(now);

                // 修改这里：将 chrono Duration 转换为 std Duration
                if wait_duration.num_seconds() > 0 {
                    let seconds = wait_duration.num_seconds() as u64;
                    let nanos = wait_duration.subsec_nanos() as u32;
                    time::sleep(StdDuration::new(seconds, nanos)).await;
                } else {
                    // 如果时间已经过了，设置一个短暂的等待
                    time::sleep(StdDuration::from_secs(1)).await;
                    continue;
                }
            }

            println!("定时任务准备触发{}", schedule_name);

            // 发送系统通知
            let _ = app_handle_clone
                .notification()
                .builder()
                .title(&schedule_name)
                .body(&schedule_description.unwrap_or("搞起搞起".to_string()))
                .show()
                .unwrap();

            // 如果是重复任务，计算下一次提醒时间
            match repeat_type.as_deref() {
                Some("weekly") => {
                    // 下一周的同一天同一时间
                    let next_time = target_time + chrono::Duration::weeks(1);
                    // 更新数据库中的时间
                    // 这里需要访问数据库，暂时省略具体实现
                    break;
                }
                Some("monthly") => {
                    // 下个月的同一天同一时间
                    let next_time = target_time + chrono::Duration::days(30); // 简化处理
                                                                              // 更新数据库中的时间
                                                                              // 这里需要访问数据库，暂时省略具体实现
                    break;
                }
                _ => {
                    // 非重复任务，任务结束
                    break;
                }
            }
        }
    });

    // 存储任务handle
    let mut tasks_map = tasks.lock().map_err(|e| format!("无法获取任务锁: {}", e))?;
    tasks_map.insert(schedule_id, handle);

    Ok(())
}
