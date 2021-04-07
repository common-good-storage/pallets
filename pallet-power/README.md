# Pallet Power

## Purpose

This pallet implements some of the [Power Actor] interfaces from the Filecoin protocol.

[Power Actor]: https://github.com/filecoin-project/specs-actors/tree/master/actors/builtin/power

## Dependencies

### Traits

This pallet does not depend on any externally defined traits.

### Pallets

This pallet depends on `pallet_common` from this repository which shares types between different pallets.

## Installation

### Runtime `Cargo.toml`

TODO: To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-power]
default-features = false
package = 'pallet-power'
git = 'https://github.com/common-good-storage/pallets'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet_power/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
impl pallet_power::Config for Runtime {
    type PeerId = Vec<u8>;
    type StoragePower = u128;
}
```

and include it in your `construct_runtime!` macro:

```rust
 Power: pallet_power::{Module, Storage},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```sh
cargo doc --open
```

