# Simple.rs
Simple.rs is a crate i made to make things in rust that i found to complicated a little easier.

Still new to rust but this was fun to make.

## The input function

Definition:
```rust
pub fn input<'a>(s: impl Into<Option<&'a str>>) -> String{

}
```

It lets you easily do input with 1 function instead of printing, flushing, reading the line, and then trimming the newline after the input