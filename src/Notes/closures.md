Closures are, essentially, functions, but they are also anonymous, meaning that they do not have names. This means that closures can be passed around into functions and structs. However, before we delve into passing closures around, let us explore closures by defining a basic closure in a blank Rust program (you can use the Rust playground if you prefer) with the following code:

```rust

fn main() {
    let test_closure = |string_input| {
        println!("{}", string_input);
    };
    test_closure("test");
}

```


We can also pass closures into a function; however, it must be noted that we can also pass functions into other functions. We can achieve passing functions into other functions with the following code:


```rust

fn add_doubles(closure: fn(i32) -> i32, one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}
fn main() {
    let closure = |int_input| {
        return int_input * 2
    };
    let outcome = add_doubles(closure, 2, 3);
    println!("{}", outcome);
}


```

Here is the breakdown for the above example:

1. *Function Definition*

```rust

fn add_doubles(closure: fn(i32) -> i32, one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}

```

This function takes a closure and two integers.  It applies the closure to each integer and sums the results.


2. *Closure Definition*

```rust

let closure = |int_input| {
    return int_input * 2
};

```

Here, 'closure' is a simple closure that takes an integer ('int_input'), doubles it, and returns the result.  The type of this closure matches 'fn(i32)-> i32', which is important for it to be passed to 'add_doubles'.


3. *Using the closure*

```rust

let outcome = add_doubles(closure, 2, 3);
println!("{}", outcome);

```

'add_doubles' is called with the previously defined closure and two integer arguments, 2 and 3. The closure is applied to each integer:

- closure(2) returns 4
- closure(3) returns 6

The function add_doubles then adds these results together to get 10.


***Output:***

The final output printed is 10, which is the result of 4 + 6.




```rust

fn add_doubles(closure: Box<dyn Fn(i32) -> i32>, one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}
fn main() {
    let one = 2;
    let closure = move |int_input| {
        return int_input * one
    };
    let outcome = add_doubles(Box::new(closure), 2, 3);
    println!("{}", outcome);
}

```

Here, we can see that the closure function parameter has the Box<dyn Fn(i32) -> i32> signature. This means that the 'add_doubles' function is accepting closures that have implemented the Fn trait that accepted i32, and returned i32. The Box struct is a smart pointer where we have put the closure on the heap because we do not know the closure’s size at compile time. You can also see that we have utilized move when defining the closure. This is because we are using the one variable, which is outside the closure. The one variable may not live long enough; therefore, the closure takes ownership of it because we used move when defining the closure.



<u>Key Concepts</u> 

Closures and Functions: In Rust, closures can be passed as arguments to functions if they match the expected function signature. This allows for highly customizable behavior at runtime.
Type Annotations: The closure is annotated implicitly to match the fn(i32) -> i32 type. Rust's type inference is often powerful enough to deduce the correct types, but you can also explicitly annotate types if necessary.