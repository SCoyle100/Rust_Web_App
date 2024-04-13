
//this imports the mod.rs file in the /to_do directory
//which in turn gives us access to all of the structs and the enums.rs
//which gives us access to Taskstatus and its implementation block
//You can see, since we have access to mod structs, we have access to the mod.rs
//file in the /to_do/structs directory, which gives us access to base, done, and pending
//Thus, we are able to use Done and Pending


mod to_do; 
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;


fn main() {
    let done = Done::new("shopping");

    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status.stringify());

    let pending = Pending::new("laundry");

    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status.stringify());
}


