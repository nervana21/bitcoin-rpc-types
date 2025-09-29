# bitcoin-rpc-types

Type definitions for Bitcoin Core's JSON-RPC interface, designed for use in code generation and fuzzing frameworks.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-rpc-types = "0.1.0"
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

## Contributing

Contributions warmly welcome.

## License

MIT OR Apache-2.0

## Related Projects

Part of the **bitcoin-rpc-\*** ecosystem, which provides type-safe Rust for seamless, interface-level testing and development.