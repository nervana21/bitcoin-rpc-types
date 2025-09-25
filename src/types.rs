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
    /// Whether the result is required (computed from optional)
    #[serde(skip)]
    pub required: bool,
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
            required: true,
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
        Self {
            type_,
            optional,
            required: !optional,
            description,
            skip_type_check,
            key_name,
            condition,
            inner,
        }
    }

    /// Post-processes the result to update required field based on optional
    pub fn post_process(&mut self) {
        self.required = !self.optional;
        for inner in &mut self.inner {
            inner.post_process();
        }
    }
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
        let mut api_def: ApiDefinition = serde_json::from_str(&content)?;
        for method in api_def.rpcs.values_mut() {
            for result in &mut method.results {
                result.post_process();
            }
        }
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
