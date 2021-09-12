# A few things

- To define a public item(function), stick a `pub` before the `fn` keyword
- To access it, (usually in a higher directory, not lower, at least so far) use `mod ${modName}`
- A module can be of two shapes
  - General module encapsulating others. E.g with module `foo`
    File structure: `foo\mod.rs`
    Inside `mod.rs`:

```rust
// import then export (namespacing)
pub mod abc
pub mod cbs
pub mod nnn
pub fn delores()-> &str{
   return "delores"
}
```

This works recursively for `abc`, `cbs` and `nnn` and using this structure, it is accessible to the top level `main.rs` as:

```rust
// import
   mod foo
   // using foo's fnx
   foo::delores()
   // using foo's modules
   foo::abc::cba()

```

Thus far, items for export as public modules I have noticed are functions and other modules.
NOTE: everything is a module. Just that all members are private by default.

- Single file as module (needs to be on same level as importer)
  File structure mod: `n.rs`
  importer: `main.rs`

In `main.rs`:

```rust
mod n.rs
```

So yeah, that's a quick word on modules. (a working example can be found in hello_cargo)
One more thing: The entry point for rust in all projs I've seen is `main.rs`
More specialties like the `pub()` function and paths as I understand them.
Onward to p1
