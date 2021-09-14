# Arrays in rust

Arrays are a fixed-length , `stack` allocated data structure. And they can only take a single data type for all the elements. They are also strongly typed and Accessed by the familiar bracket notation with indexing.

- Note: Cannot go more or less than the defined size of the array
- Mutatable array's elements can be reassigned like in js

**Special**: You can create a reference to a variable using the `&` before the variable

- Using shorthand notation with arrays. Both in the type and the element definitions, one can use a short hand with the `type; number_occurrance` e.g `[5;13]` will fill in 13 elements as five, and same with `[i32;13]`

## Methods

- `.len()` - the usual `usize` info.

# Std functions

- One can use `size_of_val`, under `mem` which is under `std` module, to find the size of an array in bytes.
- Note:we cannot pass array directly, rather a reference to it using `&`.
- Also, apparrently we use this function to find the size dynamically. i.e when not known ahead of time. Such as slices and so on.
  - This can be typed with `<>`

# Cleaning up nested imports

Use `use` keyword to import a module or submodule or its function for use namespaced or not. Just like require.

E.g: with a nested module structure like:

SideNOte: I installed the `tree` command, and apparrently one can make subdirectories more easily in a one-liner with `mkdir`??

- A messy way is by using the `-p` flag and going `root sub subsub subsubsub`

```sh
root
├── mod.rs
├── problem05
│   ├── mod.rs
│   ├── tuples.md
│   └── tuples.rs
├── problem06
│   ├── arrays.md
│   ├── arrays.rs
│   └── mod.rs
└── problem07
    └── mod.rs

```

We can access problem07 only without repeating root by simply, at top level:

```rust
use root::problem07;

//...

problem07::arrays
```

# Slices and references.

You can make a ref to anything - type and var , using `&`.  
E.g  
`a : &[i32;5] = &some_i32_arr`.  
The above will return that ref.  
## What of slices?  
`a : &[i32;5] = &some_i32_arr[0..2]`  
The above will return the first two elements - Note: Arrays can only be printed with the debug trait.  

Use `..` from start index to length of result or how many elements should be packed from that start.  

A little note on slices: THey are typed differently as `[i32]` Read: slice of i32.

