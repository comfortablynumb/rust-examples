//! # lib-core
//!
//! A core library demonstrating Cargo features and workspace dependencies.
//!
//! ## Features
//!
//! - `json` (default): Enables JSON serialization support
//! - `extra`: Enables additional utility functions
//! - `advanced`: Enables advanced features (includes `extra`)

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

/// Core data structure
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

impl Data {
    /// Create a new Data instance
    pub fn new(id: u64, name: impl Into<String>, value: f64) -> Self {
        Self {
            id,
            name: name.into(),
            value,
        }
    }

    /// Validate the data
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(CoreError::InvalidInput("name cannot be empty".to_string()));
        }
        if self.value < 0.0 {
            return Err(CoreError::InvalidInput(
                "value cannot be negative".to_string(),
            ));
        }
        Ok(())
    }

    /// Calculate a derived value
    pub fn calculate(&self) -> f64 {
        self.value * 2.0 + f64::from(self.id as u32)
    }
}

/// JSON serialization support (only available with "json" feature)
#[cfg(feature = "json")]
pub mod json {
    use super::{Data, Result};

    /// Serialize data to JSON string
    pub fn to_json(data: &Data) -> Result<String> {
        Ok(serde_json::to_string(data)?)
    }

    /// Serialize data to pretty JSON string
    pub fn to_json_pretty(data: &Data) -> Result<String> {
        Ok(serde_json::to_string_pretty(data)?)
    }

    /// Deserialize data from JSON string
    pub fn from_json(json: &str) -> Result<Data> {
        Ok(serde_json::from_str(json)?)
    }
}

/// Extra utilities (only available with "extra" feature)
#[cfg(feature = "extra")]
pub mod extra {
    use super::Data;

    /// Batch process multiple data items
    pub fn batch_process(items: &[Data]) -> Vec<f64> {
        items.iter().map(|item| item.calculate()).collect()
    }

    /// Find the maximum value
    pub fn find_max(items: &[Data]) -> Option<f64> {
        items
            .iter()
            .map(|item| item.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Calculate the average value
    pub fn calculate_average(items: &[Data]) -> f64 {
        if items.is_empty() {
            return 0.0;
        }
        let sum: f64 = items.iter().map(|item| item.value).sum();
        sum / items.len() as f64
    }
}

/// Advanced features (only available with "advanced" feature)
#[cfg(feature = "advanced")]
pub mod advanced {
    use super::Data;

    /// Complex data analysis
    pub struct DataAnalyzer {
        items: Vec<Data>,
    }

    impl DataAnalyzer {
        pub fn new(items: Vec<Data>) -> Self {
            Self { items }
        }

        pub fn analyze(&self) -> AnalysisResult {
            let total = self.items.len();
            let sum: f64 = self.items.iter().map(|item| item.value).sum();
            let avg = if total > 0 { sum / total as f64 } else { 0.0 };

            AnalysisResult {
                count: total,
                sum,
                average: avg,
                min: self
                    .items
                    .iter()
                    .map(|item| item.value)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
                max: self
                    .items
                    .iter()
                    .map(|item| item.value)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            }
        }
    }

    #[derive(Debug)]
    pub struct AnalysisResult {
        pub count: usize,
        pub sum: f64,
        pub average: f64,
        pub min: f64,
        pub max: f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_creation() {
        let data = Data::new(1, "test", 42.0);
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "test");
        assert_eq!(data.value, 42.0);
    }

    #[test]
    fn test_validation() {
        let valid = Data::new(1, "test", 42.0);
        assert!(valid.validate().is_ok());

        let invalid_name = Data::new(1, "", 42.0);
        assert!(invalid_name.validate().is_err());

        let invalid_value = Data::new(1, "test", -1.0);
        assert!(invalid_value.validate().is_err());
    }

    #[test]
    fn test_calculate() {
        let data = Data::new(5, "test", 10.0);
        assert_eq!(data.calculate(), 25.0); // 10.0 * 2.0 + 5.0
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_serialization() {
        let data = Data::new(1, "test", 42.0);
        let json = json::to_json(&data).unwrap();
        let deserialized = json::from_json(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[cfg(feature = "extra")]
    #[test]
    fn test_extra_features() {
        let items = vec![
            Data::new(1, "a", 10.0),
            Data::new(2, "b", 20.0),
            Data::new(3, "c", 30.0),
        ];

        let results = extra::batch_process(&items);
        assert_eq!(results.len(), 3);

        let max = extra::find_max(&items);
        assert_eq!(max, Some(30.0));

        let avg = extra::calculate_average(&items);
        assert_eq!(avg, 20.0);
    }

    #[cfg(feature = "advanced")]
    #[test]
    fn test_advanced_features() {
        let items = vec![
            Data::new(1, "a", 10.0),
            Data::new(2, "b", 20.0),
            Data::new(3, "c", 30.0),
        ];

        let analyzer = advanced::DataAnalyzer::new(items);
        let result = analyzer.analyze();

        assert_eq!(result.count, 3);
        assert_eq!(result.sum, 60.0);
        assert_eq!(result.average, 20.0);
        assert_eq!(result.min, 10.0);
        assert_eq!(result.max, 30.0);
    }
}
