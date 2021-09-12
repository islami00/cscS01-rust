# main.rs Import statement:

`class01::problem01::printing::run()`

# Intro

Functions will be covered like this.

## First things first, `println!()`

- Regular args, now format specifiers

  - `{}` : a placeholder. in `println!` instances are evaluated as positional arguments.
    Specially:

  ```rust
  pub fn run(){
    println!("Hello {0} your {1} has {2} {0}!","delores","peacock","died")
  }
  ```

  i.e this is essentailly array indexing for your string subs `{$index}`

  - `{name}`: named args to the string
    `println!("nani? {name}",name="delores")`
    This allows for the same flexibility as before.
    Thus far: Named or positional arguments to `println!` string

  - format specifier form:

    - For numbers: These are also called **placeholder traits**

    `{:${format}}`, where `${format}` could be: `x`, `o`, `b` which are `hex`, `octal`, and `bin` respectively.  
     Any number passed in will be converted to its equivalent in that number system
    E.g: (Note how I used indexing along with the traits. Nice interpolation):

    In conclusion: Use placeholder traits to describe the data you want to pass in, and an indexing scheme to pass it in

```rust
  pub fn run(){
  println!("binary: {0:b} hex : {0:x} octal: {0:o}",10 );
  }
```

- debug value: `{:?}`  
  Used when you don't know how it'll look, i.e supports almost all. Including tuples.  
  I.e we can put multiple values in that.

Of course, these only work in `println!`, at least from what I have seen.

Along with this, we can also do things doable in an assignment, like arithmentic

## an official trait breakdown:

```
note: the only appropriate formatting traits are:
      - ``, which uses the `Display` trait
      - `?`, which uses the `Debug` trait
      - `e`, which uses the `LowerExp` trait
      - `E`, which uses the `UpperExp` trait
      - `o`, which uses the `Octal` trait
      - `p`, which uses the `Pointer` trait
      - `b`, which uses the `Binary` trait
      - `x`, which uses the `LowerHex` trait
      - `X`, which uses the `UpperHex` trait
help: use the `Display` trait: ``
```
