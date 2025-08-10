use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("👋 {name}!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            invoke_mood,
            get_possible_emoji,
            get_todays_moods,
            generate_sample_data,
            reset_store
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn invoke_mood(mood: String, app: tauri::AppHandle) {
    let timestamp = chrono::Utc::now();
    if let Some(mood_type) = emoji_to_mood(&mood) {
        let entry = MoodEntry {
            mood: mood_type,
            timestamp: timestamp.to_string(),
        };
        let key = timestamp.to_string();
        let value = json!(entry);
        use_store(app, &key, value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MoodEntry {
    mood: MoodType,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum MoodType {
    Happy,
    Sad,
    Neutral,
    Angry,
    Excited,
}

fn emoji_to_mood(emoji: &str) -> Option<MoodType> {
    match emoji {
        "😊" | "😀" | "🙂" | "😃" => Some(MoodType::Happy),
        "😢" | "😭" | "☹️" | "🙁" | "&#x1F641" => Some(MoodType::Sad),
        "😐" | "😑" => Some(MoodType::Neutral),
        "😠" | "😡" | "🤬" => Some(MoodType::Angry),
        "😄" | "😆" | "🤩" | "😂" => Some(MoodType::Excited),
        _ => None,
    }
}

#[tauri::command]
fn get_possible_emoji() -> Vec<String> {
    vec![
        "😊".to_string(),
        "😢".to_string(),
        "😐".to_string(),
        "😠".to_string(),
        "😄".to_string(),
    ]
}

fn use_store(app: AppHandle, key: &str, value: Value) {
    if let Ok(store) = app.store("store.json") {
        store.set(key.to_string(), value);
    }
}

#[tauri::command]
fn get_todays_moods(app: AppHandle) -> Result<Vec<MoodEntry>, String> {
    match app.store("store.json") {
        Ok(store) => {
            let entries = store.entries();
            let today = chrono::Utc::now().date_naive();
            let mut today_moods = Vec::new();

            for (_, value) in entries {
                if let Ok(entry) = serde_json::from_value::<MoodEntry>(value) {
                    // todays entry
                    if let Ok(entry_time) = entry.timestamp.parse::<chrono::DateTime<Utc>>() {
                        let entry_date = entry_time.date_naive();
                        if entry_date == today {
                            today_moods.push(entry);
                        }
                    }
                }
            }

            // Sort moods by timestamp
            today_moods.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            Ok(today_moods)
        }
        Err(_) => Err("❌".to_string()),
    }
}

#[tauri::command]
fn generate_sample_data(app: AppHandle) -> Result<String, String> {
    let sample_moods = vec![
        (MoodType::Happy, "09:00:00"),
        (MoodType::Excited, "12:30:00"),
        (MoodType::Neutral, "15:15:00"),
        (MoodType::Sad, "17:45:00"),
        (MoodType::Happy, "20:00:00"),
    ];

    let today = chrono::Utc::now().date_naive();

    for (mood, time_str) in sample_moods {
        let timestamp_str = format!("{}T{}Z", today, time_str);
        if let Ok(timestamp) = timestamp_str.parse::<chrono::DateTime<Utc>>() {
            let entry = MoodEntry {
                mood,
                timestamp: timestamp.to_rfc3339(),
            };
            let key = timestamp.to_rfc3339();
            let value = json!(entry);
            use_store(app.clone(), &key, value);
        }
    }

    Ok("✅".to_string())
}

#[tauri::command]
fn reset_store(app: AppHandle) -> Result<String, String> {
    match app.store("store.json") {
        Ok(store) => {
            store.clear();
            Ok("✅".to_string())
        }
        Err(e) => Err(format!("Failed to access store: {}", e)),
    }
}
