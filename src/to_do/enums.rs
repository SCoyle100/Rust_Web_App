use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING
}


 
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            TaskStatus::DONE => {"DONE".to_string()},
            TaskStatus::PENDING => {"PENDING".to_string()},
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}



//Calling this implementation block will enable us to print out the status of the to-do task and write it in our JSON file.
// 
// Note:  While the stringify function works, there is another way to convert the value of the enum to a string.  To achieve
// the string conversion, we can implement the Display trait for TaskStatus.  First, we must import the format module with the
// the following:





// The following implements the Display trait for the TaskStatus struct with the following code:


impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {write!(f, "DONE")},
            &Self::PENDING => {write!(f, "PENDING")}
        }
    }
}
