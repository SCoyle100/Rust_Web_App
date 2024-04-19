
The following example is to help me internalize and understand String vs &str





1. Collecting Input as String:

When you need to collect input from a user, such as a username during registration, you would use a String because the input size is unknown and might be mutable. The String type is ideal here because it can grow to accommodate whatever the user enters.

```rust

let mut username = String::new();
std::io::stdin().read_line(&mut username).expect("Failed to read line");

```


2. Using the Username as &str:

After you have collected the username and stored it in a String, you might want to use this data in various parts of your program where you don’t need to modify the username. In these cases, passing around a &str is more efficient. This is because &str is a reference and does not involve ownership transfer or cloning of the data, reducing memory allocation and improving performance.

```rust

fn greet_user(user_name: &str) {
    println!("Welcome, {}!", user_name.trim()); // Use trim to remove any newline characters
}

greet_user(&username);  // Pass a reference to the username


```



This approach allows you to efficiently manage memory and data mutability:

Memory Efficiency: Using &str avoids unnecessary cloning of String data, which is beneficial, especially if the data is passed frequently between functions.
Safety and Mutability: String allows you to modify the data when necessary (e.g., appending, replacing parts of the string), while &str provides safe, read-only access to the string data, which can prevent accidental data modification across different parts of your program.
Thus, using String for mutable and size-variable data, and converting to &str for read-only operations, leverages the strengths of both types effectively in Rust.





