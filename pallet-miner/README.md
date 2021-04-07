# Pallet Miner

## Purpose

This pallet implements the Miner Actor

## Dependencies

### Traits

This pallet depends on the `Power` trait from `pallet_commmon`

### Pallets

This pallet depends on `pallet_common` from this repository which shares types between different pallets.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-miner]
default-features = false
package = 'pallet-miner'
git = 'https://github.com/common-good-storage/pallets'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet_miner/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
parameter_types! {
    pub BlockDelay: BlockNumber = 5;
}

impl pallet_miner::Config for Runtime {
    type Event = Event;
    type Power = Power;
    type BlockDelay = BlockDelay;
}

```

and include it in your `construct_runtime!` macro:

```rust
        Miner: pallet_miner::{Module, Call, Storage, Event<T>},

```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```sh
cargo doc --open
```

