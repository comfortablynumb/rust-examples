#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Serde Serialization/Deserialization Examples
//!
//! This example demonstrates various features of the Serde library including:
//! - Basic serialization/deserialization
//! - Multiple formats (JSON, YAML, TOML, CSV)
//! - Custom serializers/deserializers
//! - Field attributes
//! - Enum representations
//! - Options and Results
//! - Complex nested structures

use serde::{Deserialize, Serialize, Deserializer, Serializer};
use std::collections::HashMap;

// ============================================================================
// 1. BASIC SERIALIZATION/DESERIALIZATION
// ============================================================================

/// Basic user structure with automatic Serialize/Deserialize implementation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    id: u64,
    username: String,
    email: String,
    age: u32,
    is_active: bool,
}

impl User {
    fn new(id: u64, username: &str, email: &str, age: u32, is_active: bool) -> Self {
        User {
            id,
            username: username.to_string(),
            email: email.to_string(),
            age,
            is_active,
        }
    }
}

// ============================================================================
// 6. CUSTOM SERIALIZATION WITH serialize_with/deserialize_with
// ============================================================================

/// Custom serializer for timestamp (converts Unix timestamp to ISO 8601 string)
fn serialize_timestamp<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // In a real application, you'd use chrono or time crate
    let formatted = format!("2025-01-01T{:02}:00:00Z", timestamp % 24);
    serializer.serialize_str(&formatted)
}

/// Custom deserializer for timestamp (converts ISO 8601 string to Unix timestamp)
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Simplified parsing - extract hour from ISO 8601 string
    let parts: Vec<&str> = s.split('T').collect();
    if parts.len() == 2 {
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if let Ok(hour) = time_parts[0].parse::<i64>() {
            return Ok(hour);
        }
    }
    Ok(0)
}

/// Custom serializer for password (always hashes/masks the value)
fn serialize_password<S>(password: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // In production, use proper hashing like bcrypt
    let _password = password; // Acknowledge the parameter
    serializer.serialize_str("***REDACTED***")
}

/// Custom deserializer for email validation
fn deserialize_validated_email<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.contains('@') {
        Ok(s)
    } else {
        Err(serde::de::Error::custom("Invalid email format"))
    }
}

// ============================================================================
// 7. FIELD ATTRIBUTES (rename, skip, default, flatten)
// ============================================================================

/// Configuration structure demonstrating various serde field attributes
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    /// Renamed field - JSON will use "app_name" instead of "application_name"
    #[serde(rename = "app_name")]
    application_name: String,

    /// Field with default value if missing during deserialization
    #[serde(default = "default_port")]
    port: u16,

    /// Skip serialization - this field won't appear in output
    #[serde(skip_serializing, default)]
    internal_cache: String,

    /// Skip deserialization - this field won't be read from input
    #[serde(skip_deserializing)]
    runtime_data: String,

    /// Skip both serialization and deserialization
    #[serde(skip)]
    temp_data: Vec<u8>,

    /// Custom serializer for timestamp
    #[serde(serialize_with = "serialize_timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    created_at: i64,

    /// Optional field - null/missing becomes None
    description: Option<String>,

    /// Flatten - nested fields appear at the same level
    #[serde(flatten)]
    metadata: Metadata,
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Metadata {
    version: String,
    author: String,
}

impl Config {
    fn new(name: &str, port: u16, created_at: i64) -> Self {
        Config {
            application_name: name.to_string(),
            port,
            internal_cache: String::new(),
            runtime_data: "runtime".to_string(),
            temp_data: vec![],
            created_at,
            description: None,
            metadata: Metadata {
                version: "1.0.0".to_string(),
                author: "Rust Developer".to_string(),
            },
        }
    }
}

// ============================================================================
// 8. ENUMS WITH DIFFERENT REPRESENTATIONS
// ============================================================================

/// Externally tagged enum (default) - uses {"VariantName": {...}}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum PaymentMethod {
    Cash,
    CreditCard { number: String, cvv: u16 },
    BankTransfer { account: String, routing: String },
}

/// Internally tagged enum - uses {"type": "VariantName", ...}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
enum Notification {
    Email { address: String, subject: String },
    Sms { phone: String, message: String },
    Push { device_id: String, title: String },
}

/// Adjacently tagged enum - uses {"tag": "VariantName", "content": {...}}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "event_type", content = "data")]
enum Event {
    UserLogin { user_id: u64, timestamp: i64 },
    UserLogout { user_id: u64, timestamp: i64 },
    Purchase { user_id: u64, amount: f64, items: Vec<String> },
}

/// Untagged enum - tries to deserialize each variant until one succeeds
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

/// Enum with renamed variants
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Status {
    Active,
    Inactive,
    PendingApproval,
    Suspended,
}

/// Enum with custom string representation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

// ============================================================================
// 9. WORKING WITH OPTIONS AND RESULTS
// ============================================================================

/// Structure demonstrating Option handling
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    username: String,

    /// Optional field - serialized as null if None
    bio: Option<String>,

    /// Skip if None - field is omitted from output when None
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,

    /// Optional with default value
    #[serde(default)]
    avatar_url: Option<String>,

    /// Optional nested structure
    settings: Option<UserSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserSettings {
    theme: String,
    notifications_enabled: bool,
    language: String,
}

/// API Response wrapper for Result-like handling
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "status", content = "data")]
enum ApiResponse<T, E> {
    #[serde(rename = "success")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ErrorDetails {
    code: u32,
    message: String,
    details: Option<String>,
}

// ============================================================================
// 10. COMPLEX NESTED STRUCTURES
// ============================================================================

/// Company structure with deeply nested data
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Company {
    id: u64,
    name: String,

    #[serde(rename = "hq")]
    headquarters: Address,

    employees: Vec<Employee>,
    departments: HashMap<String, Department>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parent_company: Option<Box<Company>>,

    financial_info: FinancialInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Employee {
    id: u64,
    name: String,
    position: String,

    #[serde(serialize_with = "serialize_password")]
    password: String,

    department_id: String,
    salary: f64,

    #[serde(default)]
    manager_id: Option<u64>,

    contact: ContactInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ContactInfo {
    #[serde(deserialize_with = "deserialize_validated_email")]
    email: String,

    phone: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emergency_contact: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Department {
    name: String,
    budget: f64,
    head_employee_id: Option<u64>,
    projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project {
    id: u64,
    name: String,
    status: Status,

    #[serde(serialize_with = "serialize_timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    start_date: i64,

    team_members: Vec<u64>,

    #[serde(flatten)]
    project_metadata: ProjectMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectMetadata {
    priority: Priority,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FinancialInfo {
    revenue: f64,
    expenses: f64,

    #[serde(skip_serializing, default)]
    tax_id: String,

    quarterly_reports: Vec<QuarterlyReport>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QuarterlyReport {
    year: u16,
    quarter: u8,
    revenue: f64,
    profit: f64,
}

// ============================================================================
// CSV SERIALIZATION STRUCTURES
// ============================================================================

/// Simple structure for CSV serialization
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    in_stock: bool,
    category: String,
}

// ============================================================================
// DEMONSTRATION FUNCTIONS
// ============================================================================

/// Demonstrates basic JSON serialization and deserialization
fn demo_json_basic() {
    println!("\n=== JSON Basic Serialization ===");

    let user = User::new(1, "alice", "alice@example.com", 28, true);

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize");
    println!("Serialized JSON: {}", json);

    // Pretty print JSON
    let json_pretty = serde_json::to_string_pretty(&user).expect("Failed to serialize");
    println!("Pretty JSON:\n{}", json_pretty);

    // Deserialize from JSON
    let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize");
    println!("Deserialized: {:?}", deserialized);

    assert_eq!(user, deserialized);
}

/// Demonstrates YAML serialization and deserialization
fn demo_yaml() {
    println!("\n=== YAML Serialization ===");

    let user = User::new(2, "bob", "bob@example.com", 35, false);

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&user).expect("Failed to serialize to YAML");
    println!("YAML:\n{}", yaml);

    // Deserialize from YAML
    let deserialized: User = serde_yaml::from_str(&yaml).expect("Failed to deserialize from YAML");
    println!("Deserialized from YAML: {:?}", deserialized);

    assert_eq!(user, deserialized);
}

/// Demonstrates TOML serialization and deserialization
fn demo_toml() {
    println!("\n=== TOML Serialization ===");

    let config = Config::new("MyApp", 3000, 10);

    // Serialize to TOML
    let toml_str = toml::to_string(&config).expect("Failed to serialize to TOML");
    println!("TOML:\n{}", toml_str);

    // Deserialize from TOML
    let deserialized: Config = toml::from_str(&toml_str).expect("Failed to deserialize from TOML");
    println!("Deserialized from TOML: {:?}", deserialized);
}

/// Demonstrates CSV serialization and deserialization
fn demo_csv() {
    println!("\n=== CSV Serialization ===");

    let products = vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            in_stock: true,
            category: "Electronics".to_string(),
        },
        Product {
            id: 2,
            name: "Mouse".to_string(),
            price: 29.99,
            in_stock: true,
            category: "Electronics".to_string(),
        },
        Product {
            id: 3,
            name: "Desk".to_string(),
            price: 299.99,
            in_stock: false,
            category: "Furniture".to_string(),
        },
    ];

    // Serialize to CSV
    let mut wtr = csv::Writer::from_writer(vec![]);
    for product in &products {
        wtr.serialize(product).expect("Failed to write CSV record");
    }
    let csv_data = String::from_utf8(wtr.into_inner().expect("Failed to get CSV data"))
        .expect("Failed to convert to string");
    println!("CSV:\n{}", csv_data);

    // Deserialize from CSV
    let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
    let mut deserialized_products = Vec::new();
    for result in rdr.deserialize() {
        let product: Product = result.expect("Failed to deserialize CSV record");
        deserialized_products.push(product);
    }
    println!("Deserialized {} products from CSV", deserialized_products.len());
}

/// Demonstrates custom serialization with serialize_with/deserialize_with
fn demo_custom_serialization() {
    println!("\n=== Custom Serialization ===");

    let config = Config::new("CustomApp", 8080, 15);

    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize");
    println!("Config with custom timestamp serialization:\n{}", json);

    let deserialized: Config = serde_json::from_str(&json).expect("Failed to deserialize");
    println!("Deserialized timestamp: {}", deserialized.created_at);
}

/// Demonstrates field attributes
fn demo_field_attributes() {
    println!("\n=== Field Attributes ===");

    let mut config = Config::new("AttributeDemo", 3000, 12);
    config.description = Some("Demo application".to_string());
    config.internal_cache = "This won't be serialized".to_string();
    config.temp_data = vec![1, 2, 3, 4, 5];

    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize");
    println!("Config with attributes:\n{}", json);
    println!("Note: 'internal_cache' and 'temp_data' are not in the output");
    println!("Note: 'application_name' is renamed to 'app_name'");
    println!("Note: 'metadata' fields are flattened to root level");
}

/// Demonstrates enum representations
fn demo_enum_representations() {
    println!("\n=== Enum Representations ===");

    // Externally tagged (default)
    let payment = PaymentMethod::CreditCard {
        number: "1234-5678-9012-3456".to_string(),
        cvv: 123,
    };
    let json = serde_json::to_string_pretty(&payment).expect("Failed to serialize");
    println!("Externally tagged enum:\n{}", json);

    // Internally tagged
    let notification = Notification::Email {
        address: "user@example.com".to_string(),
        subject: "Welcome!".to_string(),
    };
    let json = serde_json::to_string_pretty(&notification).expect("Failed to serialize");
    println!("\nInternally tagged enum:\n{}", json);

    // Adjacently tagged
    let event = Event::Purchase {
        user_id: 42,
        amount: 99.99,
        items: vec!["Item1".to_string(), "Item2".to_string()],
    };
    let json = serde_json::to_string_pretty(&event).expect("Failed to serialize");
    println!("\nAdjacently tagged enum:\n{}", json);

    // Untagged
    let value = Value::Object({
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::Text("John".to_string()));
        map.insert("age".to_string(), Value::Number(30.0));
        map.insert("active".to_string(), Value::Bool(true));
        map
    });
    let json = serde_json::to_string_pretty(&value).expect("Failed to serialize");
    println!("\nUntagged enum:\n{}", json);

    // Renamed variants
    let status = Status::PendingApproval;
    let json = serde_json::to_string(&status).expect("Failed to serialize");
    println!("\nRenamed variants (snake_case): {}", json);

    let log_level = LogLevel::Warning;
    let json = serde_json::to_string(&log_level).expect("Failed to serialize");
    println!("Renamed variants (UPPERCASE): {}", json);
}

/// Demonstrates working with Options
fn demo_options() {
    println!("\n=== Working with Options ===");

    let profile1 = Profile {
        username: "developer".to_string(),
        bio: Some("Rust enthusiast".to_string()),
        website: Some("https://example.com".to_string()),
        avatar_url: None,
        settings: Some(UserSettings {
            theme: "dark".to_string(),
            notifications_enabled: true,
            language: "en".to_string(),
        }),
    };

    let json1 = serde_json::to_string_pretty(&profile1).expect("Failed to serialize");
    println!("Profile with Some values:\n{}", json1);

    let profile2 = Profile {
        username: "minimalist".to_string(),
        bio: None,
        website: None,
        avatar_url: None,
        settings: None,
    };

    let json2 = serde_json::to_string_pretty(&profile2).expect("Failed to serialize");
    println!("\nProfile with None values:\n{}", json2);
    println!("Note: 'website' field is omitted when None due to skip_serializing_if");
}

/// Demonstrates API response with Result-like handling
fn demo_api_responses() {
    println!("\n=== API Response Handling ===");

    // Success response
    let success_response: ApiResponse<User, ErrorDetails> = ApiResponse::Ok(
        User::new(1, "alice", "alice@example.com", 28, true)
    );
    let json = serde_json::to_string_pretty(&success_response).expect("Failed to serialize");
    println!("Success response:\n{}", json);

    // Error response
    let error_response: ApiResponse<User, ErrorDetails> = ApiResponse::Err(ErrorDetails {
        code: 404,
        message: "User not found".to_string(),
        details: Some("The requested user ID does not exist".to_string()),
    });
    let json = serde_json::to_string_pretty(&error_response).expect("Failed to serialize");
    println!("\nError response:\n{}", json);
}

/// Demonstrates complex nested structures
fn demo_complex_structures() {
    println!("\n=== Complex Nested Structures ===");

    let mut departments = HashMap::new();
    departments.insert(
        "engineering".to_string(),
        Department {
            name: "Engineering".to_string(),
            budget: 500000.0,
            head_employee_id: Some(101),
            projects: vec![
                Project {
                    id: 1,
                    name: "Project Alpha".to_string(),
                    status: Status::Active,
                    start_date: 8,
                    team_members: vec![101, 102, 103],
                    project_metadata: ProjectMetadata {
                        priority: Priority::High,
                        tags: vec!["backend".to_string(), "api".to_string()],
                    },
                },
                Project {
                    id: 2,
                    name: "Project Beta".to_string(),
                    status: Status::PendingApproval,
                    start_date: 14,
                    team_members: vec![102, 104],
                    project_metadata: ProjectMetadata {
                        priority: Priority::Medium,
                        tags: vec!["frontend".to_string(), "ui".to_string()],
                    },
                },
            ],
        },
    );

    let company = Company {
        id: 1,
        name: "Tech Corp".to_string(),
        headquarters: Address {
            street: "123 Tech Street".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip_code: "94102".to_string(),
            country: "USA".to_string(),
        },
        employees: vec![
            Employee {
                id: 101,
                name: "Alice Johnson".to_string(),
                position: "Senior Engineer".to_string(),
                password: "secure_password_123".to_string(),
                department_id: "engineering".to_string(),
                salary: 120000.0,
                manager_id: None,
                contact: ContactInfo {
                    email: "alice@techcorp.com".to_string(),
                    phone: "+1-555-0101".to_string(),
                    emergency_contact: Some("+1-555-0199".to_string()),
                },
            },
            Employee {
                id: 102,
                name: "Bob Smith".to_string(),
                position: "Engineer".to_string(),
                password: "another_secure_pass".to_string(),
                department_id: "engineering".to_string(),
                salary: 95000.0,
                manager_id: Some(101),
                contact: ContactInfo {
                    email: "bob@techcorp.com".to_string(),
                    phone: "+1-555-0102".to_string(),
                    emergency_contact: None,
                },
            },
        ],
        departments,
        parent_company: None,
        financial_info: FinancialInfo {
            revenue: 2000000.0,
            expenses: 1500000.0,
            tax_id: "XX-XXXXXXX".to_string(),
            quarterly_reports: vec![
                QuarterlyReport {
                    year: 2025,
                    quarter: 1,
                    revenue: 500000.0,
                    profit: 125000.0,
                },
                QuarterlyReport {
                    year: 2025,
                    quarter: 2,
                    revenue: 550000.0,
                    profit: 137500.0,
                },
            ],
        },
    };

    let json = serde_json::to_string_pretty(&company).expect("Failed to serialize");
    println!("Complex company structure:\n{}", json);

    // Demonstrate deserialization round-trip
    let deserialized: Company = serde_json::from_str(&json).expect("Failed to deserialize");
    println!("\nSuccessfully deserialized company: {}", deserialized.name);
    println!("Total employees: {}", deserialized.employees.len());
    println!("Total departments: {}", deserialized.departments.len());
}

/// Demonstrates working with multiple formats for the same data
fn demo_multi_format() {
    println!("\n=== Multi-Format Serialization ===");

    let user = User::new(100, "charlie", "charlie@example.com", 42, true);

    // JSON
    let json = serde_json::to_string_pretty(&user).expect("Failed to serialize to JSON");
    println!("JSON format:\n{}\n", json);

    // YAML
    let yaml = serde_yaml::to_string(&user).expect("Failed to serialize to YAML");
    println!("YAML format:\n{}\n", yaml);

    // TOML (requires wrapping in a table)
    #[derive(Serialize)]
    struct UserWrapper {
        user: User,
    }
    let wrapper = UserWrapper { user: user.clone() };
    let toml_str = toml::to_string(&wrapper).expect("Failed to serialize to TOML");
    println!("TOML format:\n{}", toml_str);
}

/// Demonstrates error handling during deserialization
fn demo_error_handling() {
    println!("\n=== Error Handling ===");

    // Missing required field
    let invalid_json = r#"{"id": 1, "username": "test"}"#;
    let result: Result<User, _> = serde_json::from_str(invalid_json);
    match result {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error (missing fields): {}", e),
    }

    // Type mismatch
    let invalid_json = r#"{"id": "not_a_number", "username": "test", "email": "test@example.com", "age": 25, "is_active": true}"#;
    let result: Result<User, _> = serde_json::from_str(invalid_json);
    match result {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error (type mismatch): {}", e),
    }

    // Valid JSON
    let valid_json = r#"{"id": 1, "username": "test", "email": "test@example.com", "age": 25, "is_active": true}"#;
    let result: Result<User, _> = serde_json::from_str(valid_json);
    match result {
        Ok(user) => println!("Successfully deserialized: {:?}", user),
        Err(e) => println!("Unexpected error: {}", e),
    }
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   Comprehensive Serde Serialization/Deserialization Demo    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // 1. Basic JSON serialization
    demo_json_basic();

    // 2. YAML serialization
    demo_yaml();

    // 3. TOML serialization
    demo_toml();

    // 4. CSV serialization
    demo_csv();

    // 5. Custom serialization
    demo_custom_serialization();

    // 6. Field attributes
    demo_field_attributes();

    // 7. Enum representations
    demo_enum_representations();

    // 8. Working with Options
    demo_options();

    // 9. API Response handling
    demo_api_responses();

    // 10. Complex nested structures
    demo_complex_structures();

    // Additional demonstrations
    demo_multi_format();
    demo_error_handling();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    Demo Complete!                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
