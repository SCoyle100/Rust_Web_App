//Now, we can build the Pending and Done structs.  
// This is when we use composition to utilize the Base struct in our /to_do/structs/pending.rs
// file with the following code:


use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base}
    }
}