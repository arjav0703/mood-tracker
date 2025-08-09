#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            invoke_mood,
            get_possible_emoji
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn invoke_mood(mood: String) {
    let timestamp = chrono::Utc::now();
    log::info!("Mood received: {mood}");
    if let Some(mood_type) = emoji_to_mood(&mood) {
        let entry = MoodEntry {
            mood: mood_type,
            timestamp,
        };
        // save to database or file (not implemented)
        log::info!(
            "Mood entry created at {}: {:?}",
            entry.timestamp,
            entry.mood
        );
    } else {
        println!("Unknown mood emoji: {mood}");
    }
    // panic!("Fuck you");
}

struct MoodEntry {
    mood: MoodType,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
enum MoodType {
    Happy,
    Sad,
    Neutral,
    Angry,
    Excited,
}

fn emoji_to_mood(emoji: &str) -> Option<MoodType> {
    match emoji {
        "😊" => Some(MoodType::Happy),
        "😢" => Some(MoodType::Sad),
        "😐" => Some(MoodType::Neutral),
        "😠" => Some(MoodType::Angry),
        "😄" => Some(MoodType::Excited),
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
