use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A user struct that can be serialized/deserialized across JS boundary
#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct User {
    #[wasm_bindgen(readonly)]
    pub id: u32,
    name: String,
    email: String,
    active: bool,
}

#[wasm_bindgen]
impl User {
    /// Create a new user
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, name: String, email: String) -> User {
        User {
            id,
            name,
            email,
            active: true,
        }
    }

    /// Get user's name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Set user's name
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get user's email
    #[wasm_bindgen(getter)]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    /// Set user's email
    #[wasm_bindgen(setter)]
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    /// Check if user is active
    #[wasm_bindgen(getter)]
    pub fn active(&self) -> bool {
        self.active
    }

    /// Activate user
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate user
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Get a greeting message
    pub fn greet(&self) -> String {
        format!("Hello, {}! Your email is {}", self.name, self.email)
    }
}

/// Statistics calculator with various mathematical operations
#[wasm_bindgen]
pub struct Statistics {
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Statistics {
    /// Create new Statistics instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Statistics {
        Statistics { data: Vec::new() }
    }

    /// Add a value to the dataset
    pub fn add(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Add multiple values from a JavaScript array
    pub fn add_many(&mut self, values: Vec<f64>) {
        self.data.extend(values);
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get count of values
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Calculate mean
    pub fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
    }

    /// Calculate median
    pub fn median(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }

        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            Some((sorted[mid - 1] + sorted[mid]) / 2.0)
        } else {
            Some(sorted[mid])
        }
    }

    /// Calculate standard deviation
    pub fn std_dev(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }

        let mean = self.mean()?;
        let variance =
            self.data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.data.len() as f64;

        Some(variance.sqrt())
    }

    /// Get minimum value
    pub fn min(&self) -> Option<f64> {
        self.data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
    }

    /// Get maximum value
    pub fn max(&self) -> Option<f64> {
        self.data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
    }
}

/// Image processing utilities
#[wasm_bindgen]
pub struct ImageProcessor;

#[wasm_bindgen]
impl ImageProcessor {
    /// Grayscale conversion
    pub fn grayscale(pixels: &mut [u8]) {
        for chunk in pixels.chunks_mut(4) {
            let avg = ((chunk[0] as u16 + chunk[1] as u16 + chunk[2] as u16) / 3) as u8;
            chunk[0] = avg;
            chunk[1] = avg;
            chunk[2] = avg;
        }
    }

    /// Invert colors
    pub fn invert(pixels: &mut [u8]) {
        for chunk in pixels.chunks_mut(4) {
            chunk[0] = 255 - chunk[0];
            chunk[1] = 255 - chunk[1];
            chunk[2] = 255 - chunk[2];
        }
    }

    /// Adjust brightness
    pub fn brightness(pixels: &mut [u8], factor: f32) {
        for chunk in pixels.chunks_mut(4) {
            chunk[0] = ((chunk[0] as f32 * factor).min(255.0).max(0.0)) as u8;
            chunk[1] = ((chunk[1] as f32 * factor).min(255.0).max(0.0)) as u8;
            chunk[2] = ((chunk[2] as f32 * factor).min(255.0).max(0.0)) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name(), "Alice");
        assert!(user.active());
    }

    #[test]
    fn test_statistics_mean() {
        let mut stats = Statistics::new();
        stats.add_many(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(stats.mean(), Some(3.0));
    }

    #[test]
    fn test_statistics_median() {
        let mut stats = Statistics::new();
        stats.add_many(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(stats.median(), Some(3.0));
    }
}

#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_user_in_browser() {
        let mut user = User::new(1, "Bob".to_string(), "bob@test.com".to_string());
        assert_eq!(user.name(), "Bob");
        user.set_name("Bobby".to_string());
        assert_eq!(user.name(), "Bobby");
    }

    #[wasm_bindgen_test]
    fn test_statistics_in_browser() {
        let mut stats = Statistics::new();
        stats.add_many(vec![10.0, 20.0, 30.0]);
        assert_eq!(stats.mean(), Some(20.0));
    }
}
