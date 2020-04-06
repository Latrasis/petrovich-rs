Petrovich-rs
==========

Petrovich is library which inflects Russian names to given grammatical case. It supports first names, last names and middle names inflections.

Petrovich-rs is Rust implementation of [Petrovich](https://github.com/rocsci/petrovich) ruby gem.

## Usage

This crate is [on crates.io](https://crates.io/crates/petrovich) and can be
used by adding `petrovich` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]

petrovich = "0.2"
```

# Examples

```rust
use petrovich::*;

fn main() {
    assert_eq!(firstname(Gender::Male, "Саша", Case::Dative), "Саше");
    assert_eq!(firstname(Gender::Female, "Изабель", Case::Genitive), "Изабель");

    assert_eq!(lastname(Gender::Male, "Станкевич", Case::Prepositional), "Станкевиче");
    assert_eq!(lastname(Gender::Female, "Станкевич", Case::Prepositional), "Станкевич");

    assert_eq!(middlename(Gender::Male, "Сергеич", Case::Instrumental), "Сергеичем");
    assert_eq!(middlename(Gender::Female, "Прокопьевна", Case::Accusative), "Прокопьевну");
}
```
