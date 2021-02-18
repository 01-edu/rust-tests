mod error;
pub use error::{ParseErr, ReadErr};
pub use std::error::Error;

use json::{parse, stringify};
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

impl Task {
    pub fn new(id: u32, description: String, level: u32) -> Task {
        Task {
            id,
            description,
            level,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}
impl TodoList {
    pub fn new(title: String, tasks: Vec<Task>) -> TodoList {
        TodoList { title, tasks }
    }
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
    let parset = parse(todo_str).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
    if parset["tasks"].is_empty() {
        return Err(ParseErr::Empty.into());
    }
    let mut v = vec![];
    for i in 0..parset["tasks"].len() {
        let a = &parset["tasks"][i];
        let task = stringify(a["description"].clone());
        v.push(Task::new(
            stringify(a["id"].clone()).parse().unwrap(),
            task.get(1..task.len() - 1).unwrap().to_string(),
            stringify(a["level"].clone()).parse().unwrap(),
        ));
    }
    let title = stringify(parset["title"].clone());
    let todo_list = TodoList::new(title.get(1..title.len() - 1).unwrap().to_string(), v);
    Ok(todo_list)
}
