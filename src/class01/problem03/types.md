# Primitive types

1. Integers `ix` or `ux`

* Unsigned and signed integers of various bits
* Integers: `u8`,`i8`,`u16`,`i16`,`u32`,`i32`,`u64`,`i64`,`u128`,`i128`, bits each - default i32

* Float: `f32`, `f64` - default `f64`
* boolean `bool`
* characters `char` - single character
* Tuples: 
* Arrays - they are of fixed length
  * Use vectors as they are growabl
* Strings are weird*

2. Type inferring: State types as needed before compile or let rust figure it out
3. characters

The `char` data type uses unicode by default to express a single character. It is denoted by a single quote  
This means that just like js, anything flies - including emoji. If you can find them that is.  
Using character escaping: `"\u{1F600}"` for example