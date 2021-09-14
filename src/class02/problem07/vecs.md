# Vectors in rust

- Essentially resizable arrays.
- Defined the same way, except the typing and a prefix:

```rust
 let mut arr: Vec<i32> = vec![1, 3, 4, 5, 6];
```

# Vectors are similar to arrays in terms of meths

- It seems they are an unstable feature,
- Also, take up more space than allocated as I started off with 5 elements but the space query gave 24bytes instead of 20.

## Vectors are mutable!

- push, push, `push`!
  - `vect.push`
  - Also weird: Despite pushing two elements on, it only occupies 24bytes
  - THis panics if what we push exceeds max size
- `pop` ! - see?, it's a stack!.
  - pop off the last element and return it or `none` if empty.

# Looping through the vector's values.

- vectors support for .. in loops via the `.iter()` method which returns an iterable
- using that method we have access to immutable elements.
- To mutate them like regular vars: `.iter_mut()`
  - this gives a reference to work with - _new concept!_.
  - TO use an assignment combo, use `*` to retrieve it's reference?
