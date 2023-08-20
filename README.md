# Cfg Matrix
Procedural macro to generate permutations of supertraits based on cfg flags.

## How this works

Put simply:

This:
```rust
#[cfg_matrix {
    SomeBound: feature = "somefeature",
    OtherBound: feature = "otherfeature"
}]
pub trait SomeTrait {}
```

Expands to this:
```rust

#[cfg(all(feature = "somefeature", feature = "otherfeature"))]
pub trait SomeTrait: SomeBound + OtherBound {}

#[cfg(all(feature = "somefeature", not(feature = "otherfeature")))]
pub trait SomeTrait: SomeBound {}

#[cfg(all(not(feature = "somefeature"), feature = "otherfeature"))]
pub trait SomeTrait: OtherBound {}

#[cfg(all(not(feature = "somefeature"), not(feature = "otherfeature")))]
pub trait SomeTrait {}
```

## Why is this needed?

Rust does not support macros in generic parameters or in type bounds. Why this is the case is unclear, but currently, Cfg Matrix can be used to solve this problem.

This does not compile:
```rust
struct SomeStruct<A: #[cfg(feature = "somefeature")] SomeBound + #[cfg(feature = "otherfeature")] OtherBound> {...}
```

But this does:
```rust
struct SomeStruct<A: SomeTrait> {...}

#[cfg(all(feature = "somefeature", feature = "otherfeature"))]
pub trait SomeTrait: SomeBound + OtherBound {}

#[cfg(all(feature = "somefeature", not(feature = "otherfeature")))]
pub trait SomeTrait: SomeBound {}

#[cfg(all(not(feature = "somefeature"), feature = "otherfeature"))]
pub trait SomeTrait: OtherBound {}

#[cfg(all(not(feature = "somefeature"), not(feature = "otherfeature")))]
pub trait SomeTrait {}
```

So Cfg Matrix provides a way to generate this code easily.

## Notes
Cfg Matrix currently has a maximum of 4 parameters. This is to prevent insane compile times from the use of this crate. Because each feature can be enabled or disabled, there 2^n different trait definitions. For 4 parameters, this is 16, but for 5 this doubles to 32. 6 is 64, 7 is 128, and 8 is 256. You get the point. If more features are needed, create two different traits, use cfg matrix on both of them, and then use a third trait to combine the two.