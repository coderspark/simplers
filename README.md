# Simple.rs
Simple.rs is a crate i made to make things in rust that i found too complicated a little easier.

Still new to rust but this was fun to make.

# Features

## The input function

Definition:
```rust
pub fn input<'a>(s: impl Into<Option<&'a str>>) -> String{

}
```

It lets you easily do input with 1 function instead of printing, flushing, reading the line, and then trimming the newline after the input

## COLOR STRUCT
Colours

+ BLACK
+ RED
+ ORANGE
+ YELLOW
+ LIGHTGREEN
+ DARKGREEN
+ MINT
+ CYAN
+ LIGHTBLUE
+ SKYBLUE
+ BLUE
+ DARKBLUE
+ DEEPPURPLE
+ PURPLE
+ VIOLET
+ MAGENTA
+ WARMPINK
+ WATERMELON
+ LIGHTGRAY
+ DARKGRAY

any others:

```rust
Color {
    r: <red>,
    g: <green>,
    b: <blue>
}
```

## Color function
Definition:
```rust
fn color(text: &'static str, clr: Color) -> String {

}
```
Color text

## Weigh function
Definition:
```rust
fn weigh(text: &'static str, wht: Weight) -> String {

}
```
Add weight to text such as **bold**, or *italic*

## Weight struct

+ BOLD
+ DIM
+ ITALIC
+ UNDERLINED
+ SLOWBLINK
+ FASTBLINK

Those are all the weights :)

## And.. Input_color

Definition:

```rust
fn color(iptext: &'static str, clr: Color) -> String {
    
}
```

input but coloured