# bitcoin-rpc-types

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/bitcoin-rpc-types)](https://docs.rs/bitcoin-rpc-types)
[![crates.io](https://img.shields.io/crates/v/bitcoin-rpc-types)](https://crates.io/crates/bitcoin-rpc-types)

Type definitions for Bitcoin Core's JSON-RPC interface.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-rpc-types = "1.0.0"
```

## Example

```rust
use bitcoin_rpc_types::{ApiDefinition, BtcMethod, BtcArgument, BtcResult, HashOrHeight};

// Load an API definition from JSON
let api_def: ApiDefinition = serde_json::from_str(json_data)?;

// Access methods
for method in api_def.methods.values() {
    println!("Method: {}", method.name);
    for arg in &method.arguments {
        println!("  Argument: {} ({})", arg.name, arg.type_name);
    }
}
```

## License

MIT OR Apache-2.0

## Related Projects

Part of the bitcoin-rpc crate ecosystem, providing type-safe Rust primitives for testing and development at the Bitcoin Core JSON-RPC interface.