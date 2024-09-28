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


Looking at our preceding code block, this might seem a little excessive, as it can be done with just two lines of code that call the do_something function and pass it to the block_on function. In this case, it is excessive, but it can give us more flexibility on how we call futures. For instance, we can call the do_something function twice and add them together as a return with the following code:


```rust

let future_three = async {
    let outcome_one = do_something(2).await;
    let outcome_two = do_something(3).await;
    return outcome_one + outcome_two
};
let future_outcome = block_on(future_three);
println!("Here is the outcome: {:?}", future_outcome);

```


Here is the output of the preceding snippet:

number 2 is running
number 3 is running
Here is the outcome: 4


Whilst the preceding output is the result that we are expecting, we know that these futures will run sequentially and that the total time for this block of code will be just above 4 seconds. Maybe we can speed this up by using join. We have seen join speed up threads by running them at the same time. It does make sense that it will also work to speed up our futures. First, we must import the join macro with the following code:


```rust

use futures::join

```

We can now utilize join for our futures and time the implementation with the following code:


```rust

let future_four = async {
    let outcome_one = do_something(2);
    let outcome_two = do_something(3);
    let results = join!(outcome_one, outcome_two);
    return results.0 + results.1
};
let now = time::Instant::now();
let result = block_on(future_four);
println!("time elapsed {:?}", now.elapsed());
println!("here is the result: {:?}", result);

```


In the preceding code, we can see that the join macro returns a tuple of the results and that we unpack the tuple to give us the same result. However, if we do run the code, we can see that although we get the result that we want, our future execution does not speed up and is still stuck at just above 4 seconds. This is because a future is not being run using an async task. We will have to use async tasks to speed up the execution of our futures. We can achieve this by carrying out the following steps:

Create the futures needed.
Put them into a vector.
Loop through the vector, spinning off tasks for each future in the vector.
Join the async tasks and sum the vector.




To join all our futures at the same time, we will have to use another crate to create our own asynchronous join function by using the async_std crate. We define this crate in the Cargo.toml file with the following code:

```rust

async-std = "1.11.0"

```


Now that we have the async_std crate, we can import what we need to carry out the approach laid out in Figure 3.2, by importing what we need at the top of the main.rs file with the following code:


```rust

use std::vec::Vec;
use async_std;
use futures::future::join_all;


```




We can now define our future with the following code:


```rust

let async_outcome = async {
    // 1.
    let mut futures_vec = Vec::new();
    let future_four = do_something(4);
    let future_five = do_something(5);
    // 2.
    futures_vec.push(future_four);
    futures_vec.push(future_five);
    // 3. 
    let handles = futures_vec.into_iter().map(
    async_std::task::spawn).collect::<Vec<_>>();
    // 4.
    let results = join_all(handles).await;
    return results.into_iter().sum::<i8>();
};

```


Here, we can see that we define our futures (1), and then we add them to our vector (2). We then loop through our futures in our vector using the into_iter function. We then spawn a thread on each future using async_std::task::spawn. This is similar to std::task::spawn. So, why bother with all this extra headache? We could just loop through the vector and spawn a thread for each task. The difference here is that the async_std::task::spawn function is spinning off an async task in the same thread. Therefore, we are concurrently running both futures in the same thread! We then join all the handles, await for these tasks to finish, and then return the sum of all these threads. Now that we have defined our async_outcome future, we can run and time it with the following code:


```rust

let now = time::Instant::now();
let result = block_on(async_outcome);
println!("time elapsed for join vec {:?}", now.elapsed());
println!("Here is the result: {:?}", result);


```


Running our additional code will give the following additional printout:


number 4 is running
number 5 is running
time elapsed for join vec 2.007713458s
Here is the result: 4





We have managed to get two async tasks running at the same time in the same thread, resulting in both futures being executed in just over 2 seconds!

As we can see, spawning threads and async tasks in Rust is straightforward. However, we must note that passing variables into threads and async tasks is not. Rust’s borrowing mechanism ensures memory safety. We must go through extra steps when passing data into a thread. Further discussion on the general concepts behind sharing data between threads is not conducive to our web project. However, we can briefly signpost what types allow us to share data:



std::sync::Arc: This type enables threads to reference outside data:


```rust

use std::sync::Arc;
use std::thread;
let names = Arc::new(vec!["dave", "chloe", "simon"]);
let reference_data = Arc::clone(&names);
    
let new_thread = thread::spawn(move || {
    println!("{}", reference_data[1]);
});

```


std::sync::Mutex: This type enables threads to mutate outside data:


```rust

use std::sync::Mutex;
use std::thread;
let count = Mutex::new(0);
    
let new_thread = thread::spawn(move || {
     count.lock().unwrap() += 1;
});


```


Inside the thread here, we dereference the result of the lock, unwrap it, and mutate it. It must be noted that the shared state can only be accessed once the lock is held.

We have now covered enough of async programming to return to our web programming. Concurrency is a subject that can be covered in an entire book, one of which is referenced in the Further reading section. For now, we must get back to exploring Rust in web development to see how our knowledge of Rust async programming affects how we understand the Actix Web server.