# Even Bigger `S`: Better String Literal

Everyone is tired of typing `"foo"._string()` and `String::from("bar")` stuff. Maybe you've tried [Big S](https://github.com/brson/big_s), a helpful crate with only one funcion in one line. With "Big S" you can do this:

```rust
assert_eq!(S("foo"), "foo".to_string())
```

Cool, but things can be better, with **Even Bigger S** and also only one macro `S`.

## Usage

### empty string:

- Big S: `S("")`
- Even Bigger S: `S!()`

### string:

- Big S: `S("foo")`
- Even Bigger S: `S!("foo")`

### concat string:

This one is new. 

- common: `"foo".to_string() + "bar"` 
- Big S: `S("foo") + "bar"`
- Even Bigger S: `S!("foo" "bar")`

If you want to separate strings with spaces, you can do:
```rust
assert_eq!(S!("hello", "world"), "hello world".to_string());
```

This one is especially helpful with an extensive string literal:

```rust
assert_eq!(
    S!(
        "Imagine how stupid it will be to include a very long paragraph literal in rust code.",
        "With even bigger s, text has much more readability.",
        "Have you ever seen a very very long String literal in your code?"
        "Well, I have. I guess I won't see that any more.",
    ),
    "Imagine how stupid it will be to include a very long paragraph literal in rust code. With even bigger s, text has much more readability. Have you ever seen a very very long String literal in your code? Well, I have. I guess I won't see that any more.".to_string()
)
```
