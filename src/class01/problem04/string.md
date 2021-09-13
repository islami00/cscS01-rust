# Types of string

1. Primitive string: `&str`
   1. Immutable, fixed-length. somewhere in **memory**
2. `String`
   1. growable, `heap-allocated` data structure - like utf-8, use when string needs to be modified
   2. you can push to it and do stuff like an array.
   3. created using `String::from` method - static?.

# methods

- They both have `.len()`
- Note: the `len()` method doesn't return number characters or glyphs, rather in bytes!
- Mutating a `String`: `.push('')` - append a `char`
- `.push_str('')` - append a primitive string
- `.capacity()` : similar to `.len()`, gets the capacity of a string in bytes
- `.is_empty`
- `.contains` : return bool
- `.replace(substr: &str,new : &str)` : non-mutating, return new String

Note: the var has to be mutable for these to work. Nicely strongly typed - what of objects?  
Also: The naming convention is like py. underscores. Little camelcase
And... escape characters work the same way. `"\n"` for line break...
And... `println!` does things on fresh lines, like `print` of py

# Looping through strings

1. For loop
   Method: `.split_whitespace`. It does this split based on `utf-8` specs. Oh yeah, this is a very advanced word counter.

# string with capacity

This allocates a buffer of specific length to the string to curb performance issue of reallocation.
Note: After allocating the buffer `String`, we can then push to it. Imagine it as a special `""`

## Difference between capacity and len

The capacity of a string is the total space it allocated to it in memory. With `Strings`, that size grows at the same rate as the len - I think...unless it works like python's list. But with a string literal, the `capacity` and `len` are fixed and the same. So far I've said a `len()` is a `capacity()` but the difference comes when you assign a `String::with_capacity(x : usize)` which makes it so that x bytes are allocated in memory for the string. In time, the string could only use 2bytes. In which case it could end up with a `len()` of 2bytes and a `capacity()` of x .

Tldr; len is dynamically calculated, capacity is a static property.

Note: Although the docs said `len()` returns bytes, I've noticed that the regular alphabet characters all consume a byte each because it has returned the correct number of letters in the string.

# Assertions - testing

We can make assertions that can fail a build using the `assert_eq!(left,right)` and `assert_ne!(left,right)` functions - For equals and not equals in the form of left == right . Note: This is aka partial equal. It takes a message too. 
Assertions do nothing if they pass.
