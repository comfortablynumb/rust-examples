// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Default)]
struct AppState {
    tasks: Mutex<Vec<Task>>,
    next_id: Mutex<usize>,
}

// Tauri commands
#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    Ok(tasks.clone())
}

#[tauri::command]
fn add_task(title: String, state: State<AppState>) -> Result<Task, String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    let mut next_id = state.next_id.lock().map_err(|e| e.to_string())?;

    let task = Task {
        id: *next_id,
        title,
        completed: false,
    };

    *next_id += 1;
    tasks.push(task.clone());

    Ok(task)
}

#[tauri::command]
fn toggle_task(id: usize, state: State<AppState>) -> Result<(), String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = !task.completed;
        Ok(())
    } else {
        Err("Task not found".to_string())
    }
}

#[tauri::command]
fn delete_task(id: usize, state: State<AppState>) -> Result<(), String> {
    let mut tasks = state.tasks.lock().map_err(|e| e.to_string())?;
    tasks.retain(|task| task.id != id);
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Tauri!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_tasks,
            add_task,
            toggle_task,
            delete_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
