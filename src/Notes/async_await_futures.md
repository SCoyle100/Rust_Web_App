<u>***Understanding async and await***</u>


The async and await syntax manages the same concepts covered in the previous section; however, there are some nuances. Instead of simply spawning off threads, we create futures and then manipulate them as and when needed.

In computer science, a future is an unprocessed computation. This is where the result is not yet available, but when we call or wait, the future will be populated with the result of the computation. Another way of describing this is that a future is a way of expressing a value that is not yet ready. As a result, a future is not exactly a thread. In fact, threads can use futures to maximize their potential. For instance, let us say that we have several network connections. We could have an individual thread for each network connection. This is better than sequentially processing all connections, as a slow network connection would prevent other faster connections from being processed down the line until it itself is processed, resulting in a slower processing time overall. However, spinning up threads for every network connection is not free. Instead, we can have a future for each network connection. These network connections can be processed by a thread from a thread pool when the future is ready. Therefore, we can see why futures are used in web programming, as there are a lot of concurrent connections.


```rust

[dependencies]
futures = "0.3.21"

use futures::executor::block_on;
use std::{thread, time};

```
We can define futures by merely using the async syntax. The 'block_on' function will block the program until the future we defined has been executed. We can now define the 'do_something' function with the following code:

```rust

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}

```

The 'do_something' function essentially does what the code says it does, which is print out what number it is, sleep for 2 seconds, and then return an integer. However, if we were to directly call it, we would not get i8. Instead, calling the 'do_something' function directly will give us Future<Output = i8>. We can run our future and time it in the main function with the following code:


```rust

fn main() {
    let now = time::Instant::now();
    let future_one = do_something(1);
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}

```


Running the preceding code will give us the following printout:


number 1 is running
time elapsed 2.00018789s
Here is the outcome: 2


we can see that our future does not execute until we apply an executor using the 'block_on' function.

This can be a bit laborious, as we may just want a future that we can execute later in the same function. We can do this with the async/await syntax. For instance, we can call the do_something function and block the code until it is finished using the await syntax inside the main function, with the following code:


```rust

let future_two = async {
    return do_something(2).await
};
let future_two = block_on(future_two);
println!("Here is the outcome: {:?}", future_two);

```