What <u>factories</u> do is abstract the module by providing an interface. While we have enjoyed building our module, if another developer wanted to use it, a simple factory interface with good documentation would save them a lot of time. All they must do is pass in a few parameters and get the constructed structs out of the factory wrapped in an enum. If we change the internals of the module or it becomes more complicated, this will not matter. If other modules use the interface, the changes would not break the rest of the code if we keep the interfaces consistent. We can build our factory by defining our factory function in the /to_do/mod.rs file with the following code:


```rust

pub mod structs;
pub mod enums;
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;
pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}
pub fn to_do_factory(title: &str, 
                     status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}

```


In the preceding code, we can see that we define an enum called ItemTypes, which packages the constructed task structs. Our factory function essentially accepts our inputted title and status. The factory then matches the inputted status. Once we have established what type of status was passed in, we build a task that matches the status and wraps it in the ItemTypes enum. This can grow and get more complicated, and our main file will be none the wiser. We can then implement this factory in our main.rs file with the following code:

```rust

mod to_do;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;
fn main() {
    let to_do_item = to_do_factory("washing", 
                                   TaskStatus::DONE);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!("{}", item.super_struct.status
                     .stringify());
            println!("{}", item.super_struct.title);
        },
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.status
                     .stringify());
            println!("{}", item.super_struct.title);
        }
    }
}

```