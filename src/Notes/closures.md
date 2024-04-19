Closures are, essentially, functions, but they are also anonymous, meaning that they do not have names. This means that closures can be passed around into functions and structs. However, before we delve into passing closures around, let us explore closures by defining a basic closure in a blank Rust program (you can use the Rust playground if you prefer) with the following code:

```rust

fn main() {
    let test_closure = |string_input| {
        println!("{}", string_input);
    };
    test_closure("test");
}

```


