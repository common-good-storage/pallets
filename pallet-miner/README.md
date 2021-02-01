# Pallet Miner 

## Purpose

This pallet implements the Miner Actor

## Dependencies

### Traits

TODO: This pallet does not depend on any externally defined traits.

### Pallets

TODO: This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

TODO: To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.substrate-pallet-template]
default_features = false
git = 'https://github.com/substrate-developer-hub/substrate-pallet-template.git'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'example_pallet/std',
]
```

### Runtime `lib.rs`

TODO: You should implement it's trait like so:

```rust
/// Used for test_module
impl example_pallet::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
ExamplePallet: substrate_pallet_template::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

TODO: This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```

