//! Core schema types for Bitcoin RPC API definitions
//!
//! Fundamental, serde-friendly types to represent Bitcoin RPC method
//! definitions, arguments, and results.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Bitcoin method argument specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcArgument {
    /// Names of the argument
    pub names: Vec<String>,
    /// Description of the argument
    pub description: String,
    /// One-line description of the argument
    #[serde(default, rename = "oneline_description")]
    pub oneline_description: String,
    /// Whether the argument can also be passed positionally
    #[serde(default, rename = "also_positional")]
    pub also_positional: bool,
    /// Type string representation
    #[serde(default, rename = "type_str")]
    pub type_str: Option<Vec<String>>,
    /// Whether the argument is required
    pub required: bool,
    /// Whether the argument is hidden from documentation
    #[serde(default)]
    pub hidden: bool,
    /// Type of the argument
    #[serde(rename = "type")]
    pub type_: String,
}

/// Bitcoin method result specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcResult {
    /// Type of the result
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether the result is optional
    #[serde(default, rename = "optional")]
    pub optional: bool,
    /// Description of the result
    pub description: String,
    /// Whether to skip type checking for this result
    #[serde(default, rename = "skip_type_check")]
    pub skip_type_check: bool,
    /// Key name for the result
    #[serde(default, rename = "key_name")]
    pub key_name: String,
    /// Condition for when this result is present
    #[serde(default)]
    pub condition: String,
    /// Inner results for nested structures
    #[serde(default)]
    pub inner: Vec<BtcResult>,
}

impl Default for BtcResult {
    /// Creates a default BtcResult with empty values
    fn default() -> Self {
        Self {
            type_: String::new(),
            optional: false,
            description: String::new(),
            skip_type_check: false,
            key_name: String::new(),
            condition: String::new(),
            inner: Vec::new(),
        }
    }
}

impl BtcResult {
    /// Creates a new BtcResult with the specified parameters
    pub fn new(
        type_: String,
        optional: bool,
        description: String,
        skip_type_check: bool,
        key_name: String,
        condition: String,
        inner: Vec<BtcResult>,
    ) -> Self {
        Self { type_, optional, description, skip_type_check, key_name, condition, inner }
    }

    /// Returns whether the result is required (computed from optional)
    pub fn required(&self) -> bool { !self.optional }
}

/// Bitcoin method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcMethod {
    /// Name of the method
    pub name: String,
    /// Description of the method
    pub description: String,
    /// Example usage of the method
    #[serde(default)]
    pub examples: String,
    /// Names of the arguments
    #[serde(default, rename = "argument_names")]
    pub argument_names: Vec<String>,
    /// Arguments for the method
    pub arguments: Vec<BtcArgument>,
    /// Results returned by the method
    pub results: Vec<BtcResult>,
}

/// A collection of all Bitcoin RPC methods and their details
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApiDefinition {
    /// List of methods sorted by the method name
    pub rpcs: BTreeMap<String, BtcMethod>,
}

impl ApiDefinition {
    /// Creates a new empty API definition
    pub fn new() -> Self { Self { rpcs: BTreeMap::new() } }

    /// Loads an API definition from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let api_def: ApiDefinition = serde_json::from_str(&content)?;
        Ok(api_def)
    }

    /// Gets a method by name
    pub fn get_method(&self, name: &str) -> Option<&BtcMethod> { self.rpcs.get(name) }
}

/// Error types for schema operations
#[derive(Error, Debug)]
pub enum SchemaError {
    /// JSON parsing error
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for schema operations
pub type Result<T> = std::result::Result<T, SchemaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_result_default() {
        let result = BtcResult::default();
        assert_eq!(result.type_, "");
        assert!(!result.optional);
        assert!(result.required());
        assert_eq!(result.description, "");
        assert!(!result.skip_type_check);
        assert_eq!(result.key_name, "");
        assert_eq!(result.condition, "");
        assert!(result.inner.is_empty());
    }

    #[test]
    fn test_btc_result_new() {
        let inner_result = BtcResult::new(
            "string".to_string(),
            true,
            "inner description".to_string(),
            false,
            "inner_key".to_string(),
            "condition".to_string(),
            vec![],
        );

        let result = BtcResult::new(
            "object".to_string(),
            false,
            "main description".to_string(),
            true,
            "main_key".to_string(),
            "main_condition".to_string(),
            vec![inner_result.clone()],
        );

        assert_eq!(result.type_, "object");
        assert!(!result.optional);
        assert!(result.required());
        assert_eq!(result.description, "main description");
        assert!(result.skip_type_check);
        assert_eq!(result.key_name, "main_key");
        assert_eq!(result.condition, "main_condition");
        assert_eq!(result.inner.len(), 1);
        assert_eq!(result.inner[0].type_, "string");
        assert!(result.inner[0].optional);
        assert!(!result.inner[0].required());
    }

    #[test]
    fn test_btc_result_required_getter() {
        let result = BtcResult {
            type_: "string".to_string(),
            optional: true,
            description: "test".to_string(),
            skip_type_check: false,
            key_name: "test_key".to_string(),
            condition: "test_condition".to_string(),
            inner: vec![BtcResult {
                type_: "number".to_string(),
                optional: false,
                description: "inner".to_string(),
                skip_type_check: false,
                key_name: "inner_key".to_string(),
                condition: "inner_condition".to_string(),
                inner: vec![],
            }],
        };

        // Main result should have required = !optional = false
        assert!(!result.required());
        assert!(result.optional);

        // Inner result should have required = !optional = true
        assert!(result.inner[0].required());
        assert!(!result.inner[0].optional);
    }

    #[test]
    fn test_api_definition_new() {
        let api_def = ApiDefinition::new();
        assert!(api_def.rpcs.is_empty());
    }

    #[test]
    fn test_api_definition_from_file() {
        use std::fs::File;
        use std::io::Write;

        // Create a temporary JSON file with results that need post-processing
        let json_content = r#"{
            "rpcs": {
                "getblock": {
                    "name": "getblock",
                    "description": "Get block information",
                    "examples": "",
                    "argument_names": ["blockhash", "verbosity"],
                    "arguments": [
                        {
                            "names": ["blockhash"],
                            "description": "The block hash",
                            "oneline_description": "",
                            "also_positional": false,
                            "type_str": null,
                            "required": true,
                            "hidden": false,
                            "type": "string"
                        }
                    ],
                    "results": [
                        {
                            "type": "object",
                            "optional": true,
                            "description": "Block information",
                            "skip_type_check": false,
                            "key_name": "",
                            "condition": "",
                            "inner": [
                                {
                                    "type": "string",
                                    "optional": false,
                                    "description": "Inner result",
                                    "skip_type_check": false,
                                    "key_name": "inner_key",
                                    "condition": "",
                                    "inner": []
                                }
                            ]
                        }
                    ]
                }
            }
        }"#;

        let temp_file = "test_api.json";
        let mut file = File::create(temp_file).unwrap();
        file.write_all(json_content.as_bytes()).unwrap();
        drop(file);

        // Test loading from file
        let api_def = ApiDefinition::from_file(temp_file).unwrap();
        assert_eq!(api_def.rpcs.len(), 1);
        assert!(api_def.rpcs.contains_key("getblock"));

        let method = api_def.rpcs.get("getblock").unwrap();
        assert_eq!(method.name, "getblock");
        assert_eq!(method.arguments.len(), 1);
        assert_eq!(method.results.len(), 1);

        // Verify results are properly computed - the main result should be optional
        assert!(!method.results[0].required());
        assert!(method.results[0].optional);

        // Verify inner results are properly computed - the inner result should be required
        assert!(method.results[0].inner[0].required());
        assert!(!method.results[0].inner[0].optional);

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_api_definition_from_file_success_path() {
        use std::fs::File;
        use std::io::Write;

        // Create a minimal JSON file to test the success path
        let json_content = r#"{
            "rpcs": {
                "simple_method": {
                    "name": "simple_method",
                    "description": "A simple method",
                    "examples": "",
                    "argument_names": [],
                    "arguments": [],
                    "results": []
                }
            }
        }"#;

        let temp_file = "test_simple_api.json";
        let mut file = File::create(temp_file).unwrap();
        file.write_all(json_content.as_bytes()).unwrap();
        drop(file);

        // Test that the success path (Ok(api_def)) is covered
        let result = ApiDefinition::from_file(temp_file);
        assert!(result.is_ok());

        let api_def = result.unwrap();
        assert_eq!(api_def.rpcs.len(), 1);
        assert!(api_def.rpcs.contains_key("simple_method"));

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_api_definition_from_file_error_cases() {
        // Test file not found error
        let result = ApiDefinition::from_file("nonexistent_file.json");
        assert!(result.is_err());
        match result.unwrap_err() {
            SchemaError::Io(_) => {} // Expected IO error
            _ => panic!("Expected IO error for nonexistent file"),
        }

        // Test invalid JSON error
        use std::fs::File;
        use std::io::Write;

        let temp_file = "test_invalid.json";
        let mut file = File::create(temp_file).unwrap();
        file.write_all(b"invalid json content").unwrap();
        drop(file);

        let result = ApiDefinition::from_file(temp_file);
        assert!(result.is_err());
        match result.unwrap_err() {
            SchemaError::JsonParse(_) => {} // Expected JSON parse error
            _ => panic!("Expected JSON parse error for invalid JSON"),
        }

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_api_definition_get_method() {
        let mut api_def = ApiDefinition::new();

        // Test getting method from empty API definition
        assert!(api_def.get_method("nonexistent").is_none());

        // Add a method
        let method = BtcMethod {
            name: "getblock".to_string(),
            description: "Get block information".to_string(),
            examples: "".to_string(),
            argument_names: vec!["blockhash".to_string()],
            arguments: vec![],
            results: vec![],
        };
        api_def.rpcs.insert("getblock".to_string(), method);

        // Test getting existing method
        let retrieved_method = api_def.get_method("getblock");
        assert!(retrieved_method.is_some());
        assert_eq!(retrieved_method.unwrap().name, "getblock");

        // Test getting non-existent method
        assert!(api_def.get_method("gettransaction").is_none());
    }
}
