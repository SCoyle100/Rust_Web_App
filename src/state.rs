//Seeing as we are going to be upgrading our storage option in the future, it makes sense to keep the operations around reading and writing to our JSON file separate from the rest of the
//application. We do not want a lot of debugging and refactoring when we pull it out for our database upgrade. We will also keep it simple as there are no schema or migrations that must 
//be managed when reading and writing to a JSON file. Considering this, all we will need are read and write functions. As our module is small and simple, we can house our module in just 
//one file next to the main.rs file. First, we need to import what we need in our src/state.rs file with the following code:

use std::fs::File;
use std::fs;
use std::io::Read;
use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;




pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();

    return state
}


