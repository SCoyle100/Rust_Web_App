pub enum TaskStatus {
    DONE,
    PENDING
}

///This enum is used for defining the status of the task.
/// 
/// If we want to write to a file or database, we are going to have to build a method to enable
/// our enum to be represented in a string format.  To do this, we can implement a stringify function 
/// for our TaskStatus enum with the following:


/* 
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }
}
*/

///Calling this implementation block will enable us to print out the status of the to-do task and write it in our JSON file.
/// 
/// Note:  While the stringify function works, there is another way to convert the value of the enum to a string.  To achieve
/// the string conversion, we can implement the Display trait for TaskStatus.  First, we must import the format module with the
/// the following:


use std::fmt;

/// The following implements the Display trait for the TaskStatus struct with the following code:


impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {write!(f, "DONE")},
            &Self::PENDING => {write!(f, "PENDING")}
        }
    }
}