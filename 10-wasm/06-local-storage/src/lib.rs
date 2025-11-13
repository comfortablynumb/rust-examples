use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Storage, console};

/// Get the localStorage object
fn local_storage() -> Result<Storage, JsValue> {
    web_sys::window()
        .ok_or_else(|| JsValue::from_str("No window object"))?
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("No localStorage"))
}

/// Get the sessionStorage object
fn session_storage() -> Result<Storage, JsValue> {
    web_sys::window()
        .ok_or_else(|| JsValue::from_str("No window object"))?
        .session_storage()?
        .ok_or_else(|| JsValue::from_str("No sessionStorage"))
}

/// Set a value in localStorage
#[wasm_bindgen]
pub fn set_item(key: &str, value: &str) -> Result<(), JsValue> {
    local_storage()?.set_item(key, value)?;
    Ok(())
}

/// Get a value from localStorage
#[wasm_bindgen]
pub fn get_item(key: &str) -> Result<Option<String>, JsValue> {
    Ok(local_storage()?.get_item(key)?)
}

/// Remove a value from localStorage
#[wasm_bindgen]
pub fn remove_item(key: &str) -> Result<(), JsValue> {
    local_storage()?.remove_item(key)?;
    Ok(())
}

/// Clear all localStorage
#[wasm_bindgen]
pub fn clear() -> Result<(), JsValue> {
    local_storage()?.clear()?;
    Ok(())
}

/// Get the number of items in localStorage
#[wasm_bindgen]
pub fn length() -> Result<u32, JsValue> {
    Ok(local_storage()?.length()?)
}

/// Get a key by index
#[wasm_bindgen]
pub fn key(index: u32) -> Result<Option<String>, JsValue> {
    Ok(local_storage()?.key(index)?)
}

/// User preferences example
#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct UserPreferences {
    theme: String,
    language: String,
    notifications: bool,
    font_size: u32,
}

#[wasm_bindgen]
impl UserPreferences {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UserPreferences {
        UserPreferences {
            theme: "light".to_string(),
            language: "en".to_string(),
            notifications: true,
            font_size: 14,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn theme(&self) -> String {
        self.theme.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }

    #[wasm_bindgen(getter)]
    pub fn language(&self) -> String {
        self.language.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    #[wasm_bindgen(getter)]
    pub fn notifications(&self) -> bool {
        self.notifications
    }

    #[wasm_bindgen(setter)]
    pub fn set_notifications(&mut self, notifications: bool) {
        self.notifications = notifications;
    }

    #[wasm_bindgen(getter)]
    pub fn font_size(&self) -> u32 {
        self.font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_size(&mut self, font_size: u32) {
        self.font_size = font_size;
    }

    /// Save preferences to localStorage
    pub fn save(&self) -> Result<(), JsValue> {
        let json = serde_json::to_string(self)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        set_item("user_preferences", &json)?;
        console::log_1(&"Preferences saved".into());
        Ok(())
    }

    /// Load preferences from localStorage
    pub fn load() -> Result<UserPreferences, JsValue> {
        match get_item("user_preferences")? {
            Some(json) => {
                let prefs: UserPreferences = serde_json::from_str(&json)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                console::log_1(&"Preferences loaded".into());
                Ok(prefs)
            }
            None => {
                console::log_1(&"No saved preferences, using defaults".into());
                Ok(UserPreferences::new())
            }
        }
    }

    /// Delete saved preferences
    pub fn delete() -> Result<(), JsValue> {
        remove_item("user_preferences")?;
        console::log_1(&"Preferences deleted".into());
        Ok(())
    }
}

/// Todo item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

/// Todo list manager
#[wasm_bindgen]
pub struct TodoList {
    items: Vec<TodoItem>,
}

#[wasm_bindgen]
impl TodoList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    /// Add a todo item
    pub fn add(&mut self, text: String) {
        let id = self.items.len() as u32;
        self.items.push(TodoItem {
            id,
            text,
            completed: false,
        });
    }

    /// Toggle a todo item
    pub fn toggle(&mut self, id: u32) -> Result<(), JsValue> {
        let item = self.items.iter_mut()
            .find(|item| item.id == id)
            .ok_or_else(|| JsValue::from_str("Item not found"))?;
        item.completed = !item.completed;
        Ok(())
    }

    /// Remove a todo item
    pub fn remove(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
    }

    /// Get all items as JSON
    pub fn get_all(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.items)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get count of items
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Get count of completed items
    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.completed).count()
    }

    /// Save to localStorage
    pub fn save(&self) -> Result<(), JsValue> {
        let json = serde_json::to_string(&self.items)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        set_item("todo_list", &json)?;
        console::log_1(&"Todo list saved".into());
        Ok(())
    }

    /// Load from localStorage
    pub fn load() -> Result<TodoList, JsValue> {
        match get_item("todo_list")? {
            Some(json) => {
                let items: Vec<TodoItem> = serde_json::from_str(&json)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                console::log_1(&"Todo list loaded".into());
                Ok(TodoList { items })
            }
            None => {
                console::log_1(&"No saved todo list".into());
                Ok(TodoList::new())
            }
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Session storage wrapper
#[wasm_bindgen]
pub struct SessionStore;

#[wasm_bindgen]
impl SessionStore {
    /// Set a value in sessionStorage
    pub fn set(key: &str, value: &str) -> Result<(), JsValue> {
        session_storage()?.set_item(key, value)?;
        Ok(())
    }

    /// Get a value from sessionStorage
    pub fn get(key: &str) -> Result<Option<String>, JsValue> {
        Ok(session_storage()?.get_item(key)?)
    }

    /// Remove a value from sessionStorage
    pub fn remove(key: &str) -> Result<(), JsValue> {
        session_storage()?.remove_item(key)?;
        Ok(())
    }

    /// Clear all sessionStorage
    pub fn clear() -> Result<(), JsValue> {
        session_storage()?.clear()?;
        Ok(())
    }
}

/// Check if localStorage is available
#[wasm_bindgen]
pub fn is_storage_available() -> bool {
    local_storage().is_ok()
}

/// Get storage usage (approximate)
#[wasm_bindgen]
pub fn get_storage_size() -> Result<usize, JsValue> {
    let storage = local_storage()?;
    let len = storage.length()?;

    let mut total_size = 0;
    for i in 0..len {
        if let Some(key) = storage.key(i)? {
            if let Some(value) = storage.get_item(&key)? {
                total_size += key.len() + value.len();
            }
        }
    }

    Ok(total_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_preferences() {
        let mut prefs = UserPreferences::new();
        assert_eq!(prefs.theme(), "light");
        prefs.set_theme("dark".to_string());
        assert_eq!(prefs.theme(), "dark");
    }

    #[test]
    fn test_todo_list() {
        let mut todos = TodoList::new();
        todos.add("Buy milk".to_string());
        todos.add("Walk dog".to_string());
        assert_eq!(todos.count(), 2);
        assert_eq!(todos.completed_count(), 0);

        todos.toggle(0).unwrap();
        assert_eq!(todos.completed_count(), 1);
    }
}
