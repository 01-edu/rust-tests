/*
## boxing_todo

### Instructions

The objective is to do an api to parse a list of *todos* that is organized in a json file,
handling all possible errors in a multiple error system.

Create a module in another file called error.rs that handles the boxing of errors.
This module must implement an `enum` called `ParseErr` that has the following elements:

- Empty
- Malformed, that has a boxing error as element

A structure called `ReadErr` that just has an element called `child_err` of type `Box<dyn Error>`.

For each data structure you will have to implement a function called `fmt` for the trait `Display` that writes
out the message **"Fail to parse todo"** in case it's a parsing error, otherwise it writes the message
**"Failed to read todo file"**. And for the `Error` trait the following functions:

- `description` that returns a string literal that says:
  - "Todo List parse failed: " for the `ParseErr`
  - "Todo List read failed: " for the `ReadErr`.

- `cause` that returns an `Option` with the error:
  - For the `ReadErr` it must just return the option with the error
  - For the `ParseErr` it will return an option that can be `None` if the tasks are **empty** otherwise the error, if
  the parsing is **malformed**.

In the main file you will have to implement a function called `get_todo` that receives a string and returns a Result
that can be the structure `TodoList` or a boxing error. This function must be able to deserialize the json file.

### Example

```rust
mod lib;
use lib::{ TodoList };

fn main() {
    let todos = TodoList::get_todo("todo_empty.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.description(), e.cause());
        }
    }
    // output : "Todo List parse failed: None"

    let todos = TodoList::get_todo("malformed_object.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.description(), e.cause().unwrap());
        }
    }
    // output : "Todo List parse failed: Malformed(Error("missing field `title`", line: 1, column: 2))"

    let todos = TodoList::get_todo("permission_err.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.description(), e.cause().unwrap());
        }
    }
    // output : "Todo List read failed: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }"
}
```

### Notions

- https://serde.rs/
- https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
- https://doc.rust-lang.org/stable/rust-by-example/trait/dyn.html
*/

mod error;
use error::{ ParseErr, ReadErr };

pub use std::error::Error;

pub use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let todo_raw = read_todos(path);
        let parsed_todos = parse_todos(&todo_raw?)?;
        Ok(parsed_todos)
    }
}

pub fn read_todos(path: &str) -> Result<String, Box<dyn Error>> {
    let raw = read_to_string(path).map_err(|e| ReadErr {
        child_err: Box::new(e),
    })?;
    Ok(raw)
}

pub fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
    let parset: TodoList =
        serde_json::from_str(&todo_str).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
    if parset.tasks.is_empty() {
        return Err(ParseErr::Empty.into());
    }
    Ok(parset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::fs;

    fn new_todo(s: String, v: Vec<Task>) -> TodoList {
        TodoList { title: s, tasks: v }
    }
    fn run(s: &TodoList, f: &str) -> Result<TodoList, Box<dyn Error>> {
        serde_json::to_writer(&File::create(f)?, s)?;
        let result = TodoList::get_todo(f);
		fs::remove_file(f)?;
        return result;
    }

    #[test]
    fn test_good_todo() {
        let file_name = "todo.json";
        let good_struct = new_todo(
            String::from("todo list for something"),
            vec![
                Task { id: 0, description: "do this".to_string(), level: 0 },
                Task { id: 1, description: "do that".to_string(), level: 5 },
            ],
        );
        let result = run(&good_struct, file_name).unwrap();

        assert_eq!(result.title, good_struct.title);
        assert_eq!(&result.tasks, &good_struct.tasks);
    }

    #[test]
    fn test_empty_tasks() {
        let file_name = "empty_tasks.json";
        let result = run(&new_todo(String::from("empty tasks"), vec![]), file_name).unwrap_err();

        assert_eq!(result.to_string(), "Failed to parses todo");
        assert_eq!(result.description(), "Todo List parse failed: ");
        assert!(!result.cause().is_some());
    }
    #[test]
    fn test_read() {
        let result = TodoList::get_todo("no_file.json").unwrap_err();

        assert_eq!(result.to_string(), "Failed to read todo file");
        assert_eq!(result.description(), "Todo List read failed: ");
    }

    #[test]
    #[should_panic(expected = "Malformed(Error(\"missing field `title`\", line: 1, column: 2))")]
    fn test_malformed_json() {
        #[derive(Serialize, Deserialize)]
        struct mal {};
        let file_name = "malformed.json";
        let malformed: mal = serde_json::from_str(r#"{}"#).unwrap();
        serde_json::to_writer(&File::create(file_name).unwrap(), &malformed).unwrap();
        let result = TodoList::get_todo(file_name);
        fs::remove_file(file_name).unwrap();

        result.unwrap_or_else(|e| panic!("{:?}", e));
    }

    #[test]
    #[should_panic(expected = "ReadErr { child_err: Os { code: 2, kind: NotFound, message: \"No such file or directory\" } }")]
    fn test_read_error() {
        TodoList::get_todo("no_file.json").unwrap_or_else(|e| panic!("{:?}", e));
    }
}
